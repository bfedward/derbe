//! The worker against a fake Resend server: sent, retried, and given-up paths.

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, Mutex,
};

use axum::{extract::State, http::HeaderMap, http::StatusCode, routing::post, Json, Router};
use derbe_backend::features::email::{
    sender::ResendClient,
    worker::{EmailWorker, MAX_ATTEMPTS},
};
use secrecy::SecretBox;
use serde_json::{json, Value};
use sqlx::PgPool;
use test_helpers::{cleanup_test_app, spawn_app};
use tokio::net::TcpListener;

mod test_helpers;

struct RecordedRequest {
    idempotency_key: Option<String>,
    authorization: Option<String>,
    body: Value,
}

#[derive(Clone)]
struct FakeResend {
    status: Arc<AtomicU16>,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

impl FakeResend {
    fn request_count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}

/// Starts a fake Resend that answers every `POST /emails` with `status`.
async fn spawn_fake_resend(status: u16) -> (String, FakeResend) {
    let fake = FakeResend {
        status: Arc::new(AtomicU16::new(status)),
        requests: Arc::default(),
    };
    let router = Router::new()
        .route("/emails", post(fake_send))
        .with_state(fake.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    (url, fake)
}

async fn fake_send(
    State(fake): State<FakeResend>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let header = |name: &str| {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
    };
    fake.requests.lock().unwrap().push(RecordedRequest {
        idempotency_key: header("idempotency-key"),
        authorization: header("authorization"),
        body,
    });
    let status = StatusCode::from_u16(fake.status.load(Ordering::SeqCst)).unwrap();
    (status, Json(json!({ "id": "fake-email-id" })))
}

fn worker(pool: &PgPool, resend_url: String) -> EmailWorker {
    let client = ResendClient::new(
        resend_url,
        SecretBox::new(Box::new("re_test".to_owned())),
        "Derbe <test@notify.derbe.app>".to_owned(),
    );
    EmailWorker::new(pool.clone(), client, "https://derbe.test".to_owned())
}

async fn queue_email(pool: &PgPool, event_key: &str, template: &str) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO email_outbox (event_key, recipient, template, payload)
         VALUES ($1, 'mandy@derbe.test', $2, $3) RETURNING id",
    )
    .bind(event_key)
    .bind(template)
    .bind(json!({
        "request_id": 7,
        "employee_name": "Emma Testerson",
        "leave_type": "Annual",
        "date_from": "2030-05-20",
        "date_to": "2030-05-24",
        "total_duration": 5.0,
        "notes": null,
    }))
    .fetch_one(pool)
    .await
    .unwrap()
}

/// (sent, attempt_count, retry_is_in_the_future, last_error)
async fn row_state(pool: &PgPool, id: i64) -> (bool, i32, bool, Option<String>) {
    sqlx::query_as(
        "SELECT sent_at IS NOT NULL, attempt_count, next_attempt_at > now(), last_error
         FROM email_outbox WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[tokio::test]
async fn a_successful_send_marks_the_row_sent_and_uses_the_event_key() {
    let app = spawn_app().await;
    let pool = &app.ctx.pool;
    let (url, fake) = spawn_fake_resend(200).await;
    let id = queue_email(pool, "leave-request:7:created", "leave_request_submitted").await;

    let worker = worker(pool, url);
    worker.process_batch().await.unwrap();

    let (sent, attempts, _, last_error) = row_state(pool, id).await;
    assert!(sent);
    assert_eq!(attempts, 0);
    assert_eq!(last_error, None);

    {
        let requests = fake.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        assert_eq!(
            request.idempotency_key.as_deref(),
            Some("leave-request:7:created")
        );
        assert_eq!(request.authorization.as_deref(), Some("Bearer re_test"));
        assert_eq!(request.body["to"], json!(["mandy@derbe.test"]));
        assert_eq!(
            request.body["subject"],
            "New leave request from Emma Testerson"
        );
        assert!(request.body["html"]
            .as_str()
            .unwrap()
            .contains("https://derbe.test/leave-request/7"));
    }

    // A sent row is never sent again.
    worker.process_batch().await.unwrap();
    assert_eq!(fake.request_count(), 1);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_server_error_stays_pending_and_is_retried_later() {
    let app = spawn_app().await;
    let pool = &app.ctx.pool;
    let (url, fake) = spawn_fake_resend(500).await;
    let id = queue_email(pool, "leave-request:7:created", "leave_request_submitted").await;
    let worker = worker(pool, url);

    worker.process_batch().await.unwrap();

    let (sent, attempts, retry_later, last_error) = row_state(pool, id).await;
    assert!(!sent);
    assert_eq!(attempts, 1);
    assert!(
        retry_later,
        "next_attempt_at should be pushed into the future"
    );
    assert!(last_error.unwrap().contains("500"));

    // Not due yet, so an immediate second batch doesn't call Resend again.
    worker.process_batch().await.unwrap();
    assert_eq!(fake.request_count(), 1);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_client_error_gives_up_without_retrying() {
    let app = spawn_app().await;
    let pool = &app.ctx.pool;
    let (url, fake) = spawn_fake_resend(422).await;
    let id = queue_email(pool, "leave-request:7:created", "leave_request_submitted").await;
    let worker = worker(pool, url);

    worker.process_batch().await.unwrap();

    let (sent, attempts, _, last_error) = row_state(pool, id).await;
    assert!(!sent);
    assert_eq!(attempts, MAX_ATTEMPTS);
    assert!(last_error.is_some());

    // Even when due, a given-up row is not claimed again.
    sqlx::query("UPDATE email_outbox SET next_attempt_at = now() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    worker.process_batch().await.unwrap();
    assert_eq!(fake.request_count(), 1);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn an_unknown_template_is_never_marked_sent() {
    let app = spawn_app().await;
    let pool = &app.ctx.pool;
    let (url, fake) = spawn_fake_resend(200).await;
    let id = queue_email(pool, "mystery:1", "does_not_exist").await;

    worker(pool, url).process_batch().await.unwrap();

    let (sent, attempts, _, last_error) = row_state(pool, id).await;
    assert!(!sent);
    assert_eq!(attempts, MAX_ATTEMPTS);
    assert!(last_error.unwrap().contains("unknown template"));
    assert_eq!(fake.request_count(), 0);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn the_worker_does_not_start_without_an_api_key() {
    let app = spawn_app().await;

    // Neither a missing nor a blank key should panic or start the loop.
    derbe_backend::features::email::worker::spawn(
        app.ctx.pool.clone(),
        None,
        "Derbe <test@notify.derbe.app>".to_owned(),
        "http://127.0.0.1:1".to_owned(),
        "https://derbe.test".to_owned(),
    );
    derbe_backend::features::email::worker::spawn(
        app.ctx.pool.clone(),
        Some(SecretBox::new(Box::new("   ".to_owned()))),
        "Derbe <test@notify.derbe.app>".to_owned(),
        "http://127.0.0.1:1".to_owned(),
        "https://derbe.test".to_owned(),
    );

    cleanup_test_app(app).await;
}
