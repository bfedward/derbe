use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    middleware,
    routing::{get, post},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::cors::{AllowCredentials, CorsLayer};

use crate::routes;
use derbe_auth::{
    handlers::{login, logout, me},
    middleware::require_auth,
};
use derbe_core::{configuration::Settings, state::AppState};
use derbe_email::worker;

#[derive(Debug, Error)]
pub enum ServeError {
    #[error("failed to connect to PostgreSQL")]
    Database(#[from] sqlx::Error),

    #[error("failed to run database migrations")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("failed to bind HTTP listener")]
    Listener(#[source] std::io::Error),

    #[error("HTTP server failed")]
    Server(#[source] std::io::Error),
}

pub async fn serve(configuration: Settings) -> Result<(), ServeError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(configuration.database_url.expose_secret())
        .await?;

    // Embedded at compile time, so the container needs no sqlx-cli to start on an empty database.
    sqlx::migrate!("./migrations").run(&pool).await?;

    worker::spawn(
        pool.clone(),
        configuration.resend_api_key,
        configuration.email_from,
        configuration.resend_base_url,
        configuration.app_base_url,
    );

    let state = AppState {
        pool,
        jwt_secret: Arc::new(configuration.jwt_secret),
        app_env: configuration.app_env,
    };

    let app = app_router(state, &configuration.cors_origins);
    let listener = TcpListener::bind((configuration.app_host.as_str(), configuration.app_port))
        .await
        .map_err(ServeError::Listener)?;

    axum::serve(listener, app).await.map_err(ServeError::Server)
}

pub fn app_router(state: AppState, cors_origins: &[String]) -> Router {
    let protected =
        routes::router().route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let public = Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me));

    Router::new()
        .merge(protected)
        .merge(public)
        .layer(get_cors_layer(cors_origins))
        .with_state(state)
}

fn get_cors_layer(cors_origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = cors_origins
        .iter()
        .map(|origin| {
            origin
                .trim()
                .parse::<HeaderValue>()
                .unwrap_or_else(|e| panic!("invalid CORS_ORIGINS entry {origin:?}: {e}"))
        })
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::COOKIE,
            header::ACCEPT,
        ])
        .allow_credentials(AllowCredentials::yes())
}

async fn health() -> &'static str {
    "Service Available!"
}
