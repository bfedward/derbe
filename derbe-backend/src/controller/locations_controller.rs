use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use derbe_shared::dto::response::location::LocationResponse;

use crate::{
    error::AppError,
    http::AppContext,
    service::{location_service::LocationService, DerbeService},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/locations", get(get_locations))
        .route("/locations/:id", get(get_location))
}

async fn get_locations(State(ctx): State<AppContext>) -> Result<Json<Vec<LocationResponse>>, AppError> {
    match LocationService::get_entities(&ctx).await {
        Ok(locations) => {
            let resps = locations.into_iter().map(LocationResponse::from).collect();
            Ok(Json(resps))
        },
        Err(e) => Err(e),
    }
}

async fn get_location(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<LocationResponse>, AppError> {
    match LocationService::get_entity_by_id(id, &ctx).await {
        Ok(location) => {
            let resp = location.into();
            Ok(Json(resp))
        },
        Err(e) => Err(e),
    }
}
