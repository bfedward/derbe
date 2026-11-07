use axum::{
    http::{HeaderName, Method},
    routing::get,
    Router,
};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{configuration::Settings, routes};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Error)]
pub enum ServeError {
    #[error("failed to connect to PostgreSQL")]
    Database(#[from] sqlx::Error),

    #[error("failed to bind HTTP listener")]
    Listener(#[source] std::io::Error),

    #[error("HTTP server failed")]
    Server(#[source] std::io::Error),
}

pub async fn serve(configuration: Settings) -> Result<(), ServeError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(configuration.database.connection_string().expose_secret())
        .await?;

    let app = app_router(AppState { pool });
    let listener = TcpListener::bind((
        configuration.application.application_host.as_str(),
        configuration.application.application_port,
    ))
    .await
    .map_err(ServeError::Listener)?;

    axum::serve(listener, app).await.map_err(ServeError::Server)
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(routes::router())
        .layer(get_cors_layer())
        .with_state(state)
}

fn get_cors_layer() -> CorsLayer {
    let content_type_header = HeaderName::from_static("content-type");

    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([content_type_header])
}

async fn health() -> &'static str {
    "Service Available!"
}
