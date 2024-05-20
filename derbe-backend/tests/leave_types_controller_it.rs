use std::net::SocketAddr;

use derbe_backend::{
    configuration::{get_configuration, DatabaseSettings},
    http::{app_router, AppContext},
};
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
async fn get_leave_types() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    println!("{}", &app.address);

    let response = client
        .get(format!("{}/leaveTypes", &app.address))
        .send()
        .await
        .expect("Failed to get leave types");

    assert!(response.status().is_success());
}
