// Each integration test crate uses a different subset of these helpers.
#![allow(dead_code)]

use std::{net::SocketAddr, str::FromStr, sync::Arc};

use derbe_backend::http::app_router;
use derbe_core::{configuration::get_configuration, model::EmployeeRole, state::AppState};
use reqwest::{
    Client, Response, StatusCode,
    header::{COOKIE, HeaderMap, HeaderValue, SET_COOKIE},
};
use secrecy::{ExposeSecret, SecretBox};
use serde_json::{Value, json};
use sqlx::{Connection, Executor, PgConnection, PgPool, postgres::PgConnectOptions};
use tokio::{
    net::TcpListener,
    sync::oneshot::{self, Sender},
};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub ctx: AppState,
    shutdown_tx: Option<Sender<()>>,
    db_name: String,
    // Same server as the app database, used to create and remove the per test database.
    admin_options: PgConnectOptions,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let configuration = get_configuration().expect("Could not get configuration!");
    let admin_options = PgConnectOptions::from_str(configuration.database_url.expose_secret())
        .expect("DATABASE_URL is not a valid Postgres URL")
        .database("postgres");
    let db_name = Uuid::new_v4().to_string();

    let db = configure_database(&admin_options, &db_name).await;

    let ctx = AppState {
        pool: db,
        jwt_secret: Arc::new(SecretBox::new(Box::new("test-secret".to_string()))),
        app_env: configuration.app_env,
    };
    let app = app_router(ctx.clone(), &configuration.cors_origins);

    // Create a shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let _ = tokio::spawn(async {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            })
            .await
            .unwrap();
    });

    TestApp {
        address: format!("http://{}", address.to_string()),
        ctx,
        shutdown_tx: Some(shutdown_tx),
        db_name,
        admin_options,
    }
}

const TEST_PASSWORD: &str = "test-password";

pub struct SeededUser {
    pub unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
}

/// Inserts an employee with the given role straight into the database.
/// Every seeded employee uses `TEST_PASSWORD`.
pub async fn seed_employee(
    app: &TestApp,
    first_name: &str,
    role: EmployeeRole,
    manager_unid: Option<Uuid>,
) -> SeededUser {
    let division_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO divisions (name) VALUES ('Seed division') ON CONFLICT DO NOTHING RETURNING id",
    )
    .fetch_optional(&app.ctx.pool)
    .await
    .expect("Failed to seed division");
    let division_id = match division_id {
        Some(id) => id,
        None => sqlx::query_scalar("SELECT id FROM divisions WHERE name = 'Seed division'")
            .fetch_one(&app.ctx.pool)
            .await
            .expect("Failed to read seeded division"),
    };

    let location_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO locations (name) VALUES ('Seed location') ON CONFLICT DO NOTHING RETURNING id",
    )
    .fetch_optional(&app.ctx.pool)
    .await
    .expect("Failed to seed location");
    let location_id = match location_id {
        Some(id) => id,
        None => sqlx::query_scalar("SELECT id FROM locations WHERE name = 'Seed location'")
            .fetch_one(&app.ctx.pool)
            .await
            .expect("Failed to read seeded location"),
    };

    let email = format!("{}@derbe.test", first_name.to_lowercase());
    let password = pwhash::bcrypt::hash(TEST_PASSWORD).expect("Failed to hash the password");

    let unid = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO employees
            (first_name, last_name, job_title, email, password, role,
             division_id, employment_type, location_id, manager_unid)
        VALUES ($1, 'Testerson', 'Tester', $2, $3, $4, $5, 'Permanent', $6, $7)
        RETURNING unid
        "#,
    )
    .bind(first_name)
    .bind(&email)
    .bind(password)
    .bind(role.to_string())
    .bind(division_id)
    .bind(location_id)
    .bind(manager_unid)
    .fetch_one(&app.ctx.pool)
    .await
    .expect("Failed to seed employee");

    SeededUser {
        unid,
        first_name: first_name.to_string(),
        last_name: String::from("Testerson"),
        role: role.to_string(),
        email,
    }
}

/// A client that sends the session cookie of the given seeded user.
pub async fn login_client(app: &TestApp, user: &SeededUser) -> Client {
    let response = Client::new()
        .post(format!("{}/login", app.address))
        .json(&serde_json::json!({ "email": user.email, "password": TEST_PASSWORD }))
        .send()
        .await
        .expect("Failed to log in");

    assert_eq!(response.status(), StatusCode::OK, "login failed");

    let set_cookie = response
        .headers()
        .get(SET_COOKIE)
        .expect("login did not set a cookie")
        .to_str()
        .unwrap();
    let session = set_cookie.split(';').next().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session).unwrap());
    Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build the client")
}

/// Seeds an employee with this role and returns them with a logged in client.
pub async fn login_as(app: &TestApp, role: EmployeeRole) -> (SeededUser, Client) {
    let user = seed_employee(app, &format!("{role}"), role, None).await;
    let client = login_client(app, &user).await;
    (user, client)
}

pub async fn configure_database(admin_options: &PgConnectOptions, db_name: &str) -> PgPool {
    let mut connection = PgConnection::connect_with(admin_options)
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{db_name}";"#).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(admin_options.clone().database(db_name))
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn cleanup_test_app(test_app: TestApp) {
    // Gracefully shutdown the server
    if let Some(shutdown_tx) = test_app.shutdown_tx {
        let _ = shutdown_tx.send(());
    }

    // Drop the connection pool before dropping the database
    drop(test_app.ctx.pool);

    // Connect to the Postgres instance without specifying a database
    let mut connection = PgConnection::connect_with(&test_app.admin_options)
        .await
        .expect("Failed to connect to Postgres");

    // Terminate any active connections to the test database
    connection
        .execute(
            format!(
                r#"
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}'
                    AND pid <> pg_backend_pid();
                    "#,
                test_app.db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to terminate active connections.");

    // Drop the test database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, test_app.db_name).as_str())
        .await
        .expect("Failed to drop database.");
}

pub async fn get_response_body_as_json(response: Response) -> serde_json::Value {
    let response_text = response.text().await.expect("Failed to get response text");
    // Parse the response text into JSON
    serde_json::from_str(&response_text).expect("Failed to parse response text to JSON")
}

pub async fn get_response_body(response: Response) -> String {
    response.text().await.expect("Failed to get response text")
}

pub async fn do_post(client: &Client, url: String, body: Value, err_msg: &str) -> Response {
    client.post(url).json(&body).send().await.expect(err_msg)
}

pub fn leave_type_body(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "default_quota": Some(10.0),
        "colour": "#FFFFFF",
        "can_book_in_future": true,
        "others_days_visibility": "ShownByDefault",
        "default_in_leave_request": true,
        "show_in_approve_days": true
    })
}

pub fn employee_body(
    first_name: &str,
    email: &str,
    division: &serde_json::Value,
    location: &serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "first_name": first_name,
        "last_name": "Testerson",
        "job_title": "Tester",
        "division_id": division["id"],
        "employment_type": "Permanent",
        "location_id": location["id"],
        "manager_unid": None::<String>,
        "email": email
    })
}

pub async fn create_division(name: &str, client: &Client, app: &TestApp) -> serde_json::Value {
    let body = serde_json::json!({
        "name": name
    });

    let response = do_post(
        client,
        format!("{}/divisions", app.address),
        body,
        "Failed to create division",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    get_response_body_as_json(response).await
}

pub async fn create_location(name: &str, client: &Client, app: &TestApp) -> serde_json::Value {
    let body = serde_json::json!({
        "name": name
    });

    let response = do_post(
        client,
        format!("{}/locations", app.address),
        body,
        "Failed to create location",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    get_response_body_as_json(response).await
}

pub async fn create_employee(
    first_name: &str,
    client: &Client,
    app: &TestApp,
) -> serde_json::Value {
    let division = create_division("Division", client, app).await;
    let location = create_location("Location", client, app).await;
    let email = format!("{}@derbe.test", first_name.to_lowercase());
    let body = employee_body(first_name, &email, &division, &location);

    let response = do_post(
        client,
        format!("{}/employees", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    // The response also carries `temporary_password`; callers want the employee.
    get_response_body_as_json(response).await["employee"].take()
}

pub async fn create_leave_type(name: &str, client: &Client, app: &TestApp) -> serde_json::Value {
    let body = leave_type_body(name);

    let response = do_post(
        client,
        format!("{}/leaveTypes", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    get_response_body_as_json(response).await
}

pub async fn post_leave_request(
    client: &Client,
    app: &TestApp,
    employee_unid: Uuid,
    leave_type_id: &Value,
) -> Response {
    let body = json!({
        "employee_unid": employee_unid,
        "leave_type_id": leave_type_id,
        "leave_status": "Pending",
        "values": {
            "date_from": "2030-05-20",
            "date_to": "2030-05-24",
            "first_day_half": false,
            "last_day_half": false,
            "total_duration": 5.0,
        },
        "date_requested": "2030-05-11T13:48:48.739+00:00",
        "date_finalised": null,
        "notes": null
    });

    client
        .post(format!("{}/leaveRequests", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave request")
}

pub async fn put_status(client: &Client, app: &TestApp, id: &Value, status: &str) -> Response {
    client
        .put(format!("{}/leaveRequests/{}/status", app.address, id))
        .json(&json!({ "leave_status": status, "notes": null }))
        .send()
        .await
        .expect("Failed to update status")
}

/// (event_key, recipient, template) for every queued email, oldest first.
pub async fn outbox_rows(app: &TestApp) -> Vec<(String, String, String)> {
    sqlx::query_as("SELECT event_key, recipient, template FROM email_outbox ORDER BY id")
        .fetch_all(&app.ctx.pool)
        .await
        .expect("Failed to read email_outbox")
}
