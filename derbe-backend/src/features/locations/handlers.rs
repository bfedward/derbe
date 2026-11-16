use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use validator::Validate;

use crate::{
    access_control::authorize,
    error::AppError,
    features::auth::middleware::AuthenticatedUser,
    features::locations::permission::LocationPermission,
    features::locations::{
        dto::{request::LocationCreate, response::LocationResponse},
        service,
    },
    http::AppState,
};

pub async fn list_locations(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    authorize(&user, LocationPermission::View)?;
    let locations = service::list(&state).await?;
    Ok(Json(
        locations.into_iter().map(LocationResponse::from).collect(),
    ))
}

pub async fn get_location(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LocationResponse>, AppError> {
    authorize(&user, LocationPermission::View)?;
    let location = service::get_by_id(id, &state).await?;
    Ok(Json(location.into()))
}

pub async fn create_location(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LocationCreate>,
) -> Result<(StatusCode, Json<LocationResponse>), AppError> {
    authorize(&user, LocationPermission::Create)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let location = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(location.into())))
}
