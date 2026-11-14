use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    auth::AuthenticatedUser,
    error::AppError,
    features::leave_types::{
        dto::{request::LeaveTypeCreate, response::LeaveTypeResponse},
        service,
    },
    http::AppState,
    permissions::{AccessController, Permission},
};

use validator::Validate;

pub async fn list_leave_types(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LeaveTypeResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveTypes)?;
    let leave_types = service::list(&state).await?;
    Ok(Json(
        leave_types
            .into_iter()
            .map(LeaveTypeResponse::from)
            .collect(),
    ))
}

pub async fn get_leave_type(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveTypeResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveTypes)?;
    let leave_type = service::get_by_id(id, &state).await?;
    Ok(Json(leave_type.into()))
}

pub async fn create_leave_type(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LeaveTypeCreate>,
) -> Result<(StatusCode, Json<LeaveTypeResponse>), AppError> {
    AccessController::authorize(&user.role, Permission::CreateLeaveType)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_type = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(leave_type.into())))
}
