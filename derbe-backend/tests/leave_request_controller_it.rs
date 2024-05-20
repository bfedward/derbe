use std::net::SocketAddr;

use derbe_backend::{
    configuration::{get_configuration, DatabaseSettings},
    http::{app_router, AppContext},
};
use reqwest::StatusCode;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub ctx: AppContext,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let mut configuration = get_configuration().expect("Could not get configuration!");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db = configure_database(&configuration.database).await;

    let ctx = AppContext { db };
    let app = app_router(&ctx);

    let _ = tokio::spawn(async {
        axum::serve(listener, app)
            // .with_graceful_shutdown(async move {
            //     _ = close_rx.await;
            // })
            .await
            .unwrap();
    });

    TestApp {
        address: format!("http://{}", address.to_string()),
        ctx,
    }
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

#[tokio::test]
async fn create_leave_request_fails_with_unknown_employee() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "id": null,
        "employee_id": 0,
        "leave_type_id": 1,
        "leave_status": "Pending",
        "date_from": "2024-05-20T00:00:00+00:00",
        "date_to": "2024-05-24T00:00:00+00:00",
        "first_day_half": false,
        "last_day_half": false,
        "total_duration": 5,
        "date_requested": "2024-05-11T13:48:48.739+00:00",
        "date_finalised": null,
        "notes": null
    });

    let response = client
        .post(format!("{}/leaveRequests", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Print the response headers
    println!("Response headers: {:#?}", response.headers());

    // Print the raw response text
    let response_text = response.text().await.expect("Failed to get response text");
    println!("Response text: {}", response_text);

    // Attempt to parse the response as JSON
    let error_message: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to parse JSON");
    assert_eq!(error_message["error"], "Employee with id 0 not found!");
}
