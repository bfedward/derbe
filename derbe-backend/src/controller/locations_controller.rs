use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use derbe_shared::dto::{request::location::LocationCreate, response::location::LocationResponse};

use crate::{error::AppError, http::AppContext, service::location_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/locations", get(get_locations))
        .route("/locations/{id}", get(get_location))
        .route("/locations", post(create_location))
}

async fn get_locations(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    match location_service::get_entities(&ctx).await {
        Ok(locations) => {
            let resps = locations.into_iter().map(LocationResponse::from).collect();
            Ok(Json(resps))
        }
        Err(e) => Err(e),
    }
}

async fn get_location(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<LocationResponse>, AppError> {
    match location_service::get_entity_by_id(id, &ctx).await {
        Ok(location) => {
            let resp = location.into();
            Ok(Json(resp))
        }
        Err(e) => Err(e),
    }
}

async fn create_location(
    State(ctx): State<AppContext>,
    Json(location): Json<LocationCreate>,
) -> Result<Response, AppError> {
    match location_service::create_entity(location, &ctx).await {
        Ok(location) => {
            let location: LocationResponse = location.into();
            let response = (StatusCode::CREATED, Json(location)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
