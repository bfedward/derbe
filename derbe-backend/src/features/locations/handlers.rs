use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    error::AppError,
    features::locations::{
        dto::{request::LocationCreate, response::LocationResponse},
        service,
    },
    http::AppState,
    permissions::{AccessController, Permission},
};

pub async fn list_locations(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLocations)?;
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
    AccessController::authorize(&user.role, Permission::ViewLocations)?;
    let location = service::get_by_id(id, &state).await?;
    Ok(Json(location.into()))
}

pub async fn create_location(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LocationCreate>,
) -> Result<(StatusCode, Json<LocationResponse>), AppError> {
    AccessController::authorize(&user.role, Permission::CreateLocation)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let location = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(location.into())))
}
