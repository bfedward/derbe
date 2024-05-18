use axum::{
    http::{HeaderName, Method},
    routing::get,
    Router,
};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    configuration::Settings,
    controller::{employee_controller, leave_request_controller, leave_types_controller},
};

#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
}

pub async fn serve(configuration: Settings) {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(configuration.database.connection_string().expose_secret())
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Couldn't establish DB connection: {}", e),
    };

    let app = app_router(AppContext { db: pool });

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    ))
    .await
    .expect("Could not init router!");

    axum::serve(listener, app)
        .await
        .expect("Could not serve app");
}

fn app_router(context: AppContext) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(leave_types_controller::router())
        .merge(leave_request_controller::router())
        .merge(employee_controller::router())
        .layer(get_cors_layer())
        .with_state(context)
}

fn get_cors_layer() -> CorsLayer {
    let content_type_header = HeaderName::from_static("content-type");

    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        // Allow the "Content-Type" header
        .allow_headers([content_type_header])
}

async fn health() -> &'static str {
    "Service Available!"
}
