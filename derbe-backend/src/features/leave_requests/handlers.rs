use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    enums::{LeaveStatus::Accepted, LeaveStatus::Pending},
    error::AppError,
    features::{
        employees,
        leave_requests::{
            dto::{
                request::{
                    AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate,
                    LeaveRequestUpdate, MyLeaveRequest,
                },
                response::{AllLeaveResponse, LeaveRequestResponse, MyLeaveResponse},
            },
            service,
        },
    },
    http::AppState,
    permissions::{AccessController, Permission},
};

pub async fn get_my_leave(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<MyLeaveRequest>,
) -> Result<Json<Vec<MyLeaveResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveRequests)?;
    let leaves = service::get_my_leave(user.unid, input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_all_leave(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<AllLeaveRequest>,
) -> Result<Json<Vec<AllLeaveResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveRequests)?;
    let leaves = service::get_all_leave(input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveRequest)?;
    let leave_request = service::get_by_id(id, &state).await?;
    Ok(Json(leave_request.into()))
}

pub async fn create_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LeaveRequestCreate>,
) -> Result<(StatusCode, Json<LeaveRequestResponse>), AppError> {
    let target = employees::service::get_access_by_id(input.employee_unid, &state).await?;
    AccessController::authorize_create_leave_request(&user, &target)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_request = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(leave_request.into())))
}

pub async fn update_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
    AccessController::authorize_update_leave_request(&user, &target)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_request = service::update(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}

pub async fn update_leave_request_status(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestStatusUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    if !leave_request.leave_status.can_become(input.leave_status) {
        return Err(AppError::ValidationFailed(format!(
            "cannot change a {} leave request to {}",
            leave_request.leave_status, input.leave_status
        )));
    }
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;

    match input.leave_status {
        Pending => AccessController::authorize_update_leave_request(&user, &target)?,
        Accepted => AccessController::authorize_approve_leave_request(&user, &target)?,
        _ => AccessController::authorize_reject_leave_request(&user, &target)?,
    }

    let leave_request = service::update_leave_request_status(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}
