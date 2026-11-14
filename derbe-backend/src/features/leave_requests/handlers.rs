use std::str::FromStr;

use axum::{
    Extension, Json, extract::{Path, State}, http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser, error::AppError, features::{employees, leave_requests::{dto::{
        request::{
            AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
            MyLeaveRequest,
        },
        response::{AllLeaveResponse, LeaveRequestResponse, MyLeaveResponse},
    }, service}}, http::AppState, permissions::AccessController,
};

pub async fn get_my_leave(
    State(state): State<AppState>,
    Json(input): Json<MyLeaveRequest>,
) -> Result<Json<Vec<MyLeaveResponse>>, AppError> {
    let leaves = service::get_my_leave(input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_all_leave(
    State(state): State<AppState>,
    Json(input): Json<AllLeaveRequest>,
) -> Result<Json<Vec<AllLeaveResponse>>, AppError> {
    let leaves = service::get_all_leave(input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_leave_request(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    Ok(Json(leave_request.into()))
}

pub async fn create_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LeaveRequestCreate>,
) -> Result<(StatusCode, Json<LeaveRequestResponse>), AppError> {
    let unid = Uuid::from_str(&input.employee_unid).map_err(|_| AppError::InvalidUuid(input.employee_unid.clone()))?;
    let target = employees::service::get_access_by_id(unid, &state).await?;
    AccessController::authorize_create_leave(&user, &target)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_request = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(leave_request.into())))
}

pub async fn update_leave_request(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_request = service::update(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}

pub async fn update_leave_request_status(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestStatusUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::update_leave_request_status(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}
