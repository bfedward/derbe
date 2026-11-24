use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use derbe_core::access_control::authorize;
use derbe_core::error::AppError;
use derbe_core::session::AuthenticatedUser;
use derbe_core::state::AppState;
use validator::Validate;

use crate::{
    permission::LocationPermission,
    {
        dto::{request::LocationCreate, response::LocationResponse},
        service,
    },
};

pub async fn list_locations(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    authorize(&user, LocationPermission::View)?;
    let locations = service::list(&state).await?;
    Ok(Json(locations))
}

pub async fn get_location(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LocationResponse>, AppError> {
    authorize(&user, LocationPermission::View)?;
    let location = service::get_by_id(id, &state).await?;
    Ok(Json(location))
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
    Ok((StatusCode::CREATED, Json(location)))
}
