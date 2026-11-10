use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    error::AppError,
    features::locations::dto::{request::LocationCreate, response::LocationResponse},
    features::locations::service,
    http::AppState,
};

pub async fn list_locations(
    State(state): State<AppState>,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    let locations = service::list(&state).await?;
    Ok(Json(
        locations.into_iter().map(LocationResponse::from).collect(),
    ))
}

pub async fn get_location(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LocationResponse>, AppError> {
    let location = service::get_by_id(id, &state).await?;
    Ok(Json(location.into()))
}

pub async fn create_location(
    State(state): State<AppState>,
    Json(input): Json<LocationCreate>,
) -> Result<(StatusCode, Json<LocationResponse>), AppError> {
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let location = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(location.into())))
}
