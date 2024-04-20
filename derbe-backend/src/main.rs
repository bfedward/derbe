use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod controller;
mod error;
mod http;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Couldn't establish DB connection: {}", e),
    };

    http::serve(pool).await
}
