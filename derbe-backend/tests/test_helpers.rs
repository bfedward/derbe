use std::net::SocketAddr;

use derbe_backend::{
    configuration::{get_configuration, DatabaseSettings, Settings},
    http::{app_router, AppContext},
};
use reqwest::{Client, Response, StatusCode};
use secrecy::ExposeSecret;
use serde_json::Value;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::{
    net::TcpListener,
    sync::oneshot::{self, Sender},
};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub ctx: AppContext,
    shutdown_tx: Option<Sender<()>>,
}

pub async fn spawn_app() -> (TestApp, Settings) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let mut configuration = get_configuration().expect("Could not get configuration!");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db = configure_database(&configuration.database).await;

    let ctx = AppContext { db };
    let app = app_router(&ctx);

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

    (
        TestApp {
            address: format!("http://{}", address.to_string()),
            ctx,
            shutdown_tx: Some(shutdown_tx),
        },
        configuration,
    )
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn cleanup_test_app(config: &DatabaseSettings, test_app: TestApp) {
    // Gracefully shutdown the server
    if let Some(shutdown_tx) = test_app.shutdown_tx {
        let _ = shutdown_tx.send(());
    }

    // Drop the connection pool before dropping the database
    drop(test_app.ctx.db);

    // Connect to the Postgres instance without specifying a database
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
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
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to terminate active connections.");

    // Drop the test database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, config.database_name).as_str())
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

    let body = serde_json::json!({
        "first_name": first_name,
        "last_name": "Testerson",
        "job_title": "Tester",
        "division_id": division["id"],
        "employment_type": "Permanent",
        "location_id": location["id"],
        "manager_id": None::<i32>,
        "email": "test@derbe.com",
        "phone": None::<String>
    });

    let response = do_post(
        client,
        format!("{}/employees", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    get_response_body_as_json(response).await
}

pub async fn create_leave_type(name: &str, client: &Client, app: &TestApp) -> serde_json::Value {
    let body = serde_json::json!({
        "name": name,
        "default_quota": Some(10.0),
        "colour": "#FFFFFF",
        "can_book_in_future": true,
        "show_in_others_days": true,
        "default_in_others_days": true,
        "default_in_leave_request": true,
        "show_in_approve_days": true
    });

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
