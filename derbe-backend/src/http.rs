use std::sync::Arc;

use axum::{
    http::{header, HeaderValue, Method},
    middleware,
    routing::{get, post},
    Router,
};
use secrecy::{ExposeSecret, SecretBox};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::cors::{AllowCredentials, CorsLayer};

use crate::{
    auth::{self, login, logout},
    configuration::{AppEnvironment, ApplicationSettings, Settings},
    routes,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<SecretBox<String>>,
    pub app_env: AppEnvironment,
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

    let app_env = configuration.application.app_env;

    let state = AppState {
        pool,
        jwt_secret: Arc::new(configuration.auth.jwt_secret),
        app_env,
    };

    let app = app_router(state, &configuration.application);
    let listener = TcpListener::bind((
        configuration.application.application_host.as_str(),
        configuration.application.application_port,
    ))
    .await
    .map_err(ServeError::Listener)?;

    axum::serve(listener, app).await.map_err(ServeError::Server)
}

pub fn app_router(state: AppState, settings: &ApplicationSettings) -> Router {
    let protected = routes::router().route_layer(middleware::from_fn_with_state(
        state.clone(),
        auth::require_auth,
    ));

    let public = Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .route("/logout", post(logout));

    Router::new()
        .merge(protected)
        .merge(public)
        .layer(get_cors_layer(settings))
        .with_state(state)
}

fn get_cors_layer(settings: &ApplicationSettings) -> CorsLayer {
    let cors_origins: Vec<HeaderValue> = settings
        .cors_origins
        .iter()
        .map(|s| {
            s.trim()
                .parse::<HeaderValue>()
                .expect("invalid CORS_ORIGINS entry {s:?}: {e}")
        })
        .collect::<Vec<HeaderValue>>();

    CorsLayer::new()
        .allow_origin(cors_origins)
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
