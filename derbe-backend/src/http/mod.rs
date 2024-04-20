use axum::{http::Method, routing::get, Router};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::controller::{leave_types_controller};

#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
}

pub async fn serve(db: PgPool) {
    let context = AppContext { db };

    let app = app_router(context);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Could not init router!");

    axum::serve(listener, app)
        .await
        .expect("Could not axum serve");
}

fn app_router(context: AppContext) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(leave_types_controller::router())
        .layer(get_cors_layer())
        .with_state(context)
}

fn get_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
}

async fn health() -> &'static str {
    "Service Available!"
}
