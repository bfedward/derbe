use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::AppError,
    http::AppContext,
    model::location::Location,
    service::{location_service::LocationService, DerbeService},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/locations", get(get_locations))
        .route("/locations/:id", get(get_location))
}

async fn get_locations(State(ctx): State<AppContext>) -> Result<Json<Vec<Location>>, AppError> {
    match LocationService::get_entities(&ctx).await {
        Ok(leave_types) => Ok(Json(leave_types)),
        Err(e) => Err(e),
    }
}

async fn get_location(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<Location>, AppError> {
    match LocationService::get_entity_by_id(id, &ctx).await {
        Ok(leave_type) => Ok(Json(leave_type)),
        Err(e) => Err(e),
    }
}
