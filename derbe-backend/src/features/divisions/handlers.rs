use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    error::AppError,
    features::divisions::{
        dto::{request::DivisionCreate, response::DivisionResponse},
        service,
    },
    http::AppState,
    permissions::{AccessController, Permission},
};

pub async fn list_divisions(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<DivisionResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewDivisions)?;
    let divisions = service::list(&state).await?;
    Ok(Json(
        divisions.into_iter().map(DivisionResponse::from).collect(),
    ))
}

pub async fn get_division(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<DivisionResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewDivisions)?;
    let division = service::get_by_id(id, &state).await?;
    Ok(Json(division.into()))
}

pub async fn create_division(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<DivisionCreate>,
) -> Result<(StatusCode, Json<DivisionResponse>), AppError> {
    AccessController::authorize(&user.role, Permission::CreateDivision)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let division = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(division.into())))
}
