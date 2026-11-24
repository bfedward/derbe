use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use derbe_core::access_control::authorize;
use derbe_core::error::AppError;
use derbe_core::session::AuthenticatedUser;
use derbe_core::state::AppState;

use crate::{
    permission::LeaveTypePermission,
    {
        dto::{request::LeaveTypeCreate, response::LeaveTypeResponse},
        service,
    },
};

use validator::Validate;

pub async fn list_leave_types(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LeaveTypeResponse>>, AppError> {
    authorize(&user, LeaveTypePermission::View)?;
    let leave_types = service::list(&state).await?;
    Ok(Json(leave_types))
}

pub async fn get_leave_type(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveTypeResponse>, AppError> {
    authorize(&user, LeaveTypePermission::View)?;
    let leave_type = service::get_by_id(id, &state).await?;
    Ok(Json(leave_type))
}

pub async fn create_leave_type(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LeaveTypeCreate>,
) -> Result<(StatusCode, Json<LeaveTypeResponse>), AppError> {
    authorize(&user, LeaveTypePermission::Create)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_type = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(leave_type)))
}
