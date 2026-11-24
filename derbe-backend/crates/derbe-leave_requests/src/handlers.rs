use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use derbe_core::{
    access_control::{authorize, authorize_on},
    error::AppError,
    model::LeaveStatus::{Accepted, Pending},
    session::AuthenticatedUser,
    state::AppState,
};
use derbe_employees as employees;
use validator::Validate;

use crate::{
    dto::{
        request::{
            AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
            MyLeaveRequest,
        },
        response::{
            AllLeaveResponse, LeaveRequestDetailsResponse, LeaveRequestResponse, MyLeaveResponse,
            TeamLeaveResponse,
        },
    },
    permission::{LeaveRequestPermission, authorize_approve, authorize_reject},
    service,
};

pub async fn get_my_leave(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<MyLeaveRequest>,
) -> Result<Json<Vec<MyLeaveResponse>>, AppError> {
    authorize(&user, LeaveRequestPermission::List)?;
    let leaves = service::get_my_leave(user.unid, input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_team_leave(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TeamLeaveResponse>>, AppError> {
    authorize(&user, LeaveRequestPermission::ListTeam)?;
    Ok(Json(service::get_team_leave(&user, &state).await?))
}

pub async fn get_all_leave(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<AllLeaveRequest>,
) -> Result<Json<Vec<AllLeaveResponse>>, AppError> {
    authorize(&user, LeaveRequestPermission::List)?;
    let leaves = service::get_all_leave(input, &state).await?;
    Ok(Json(leaves))
}

pub async fn get_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveRequestDetailsResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
    authorize_on(&user, &target, LeaveRequestPermission::View)?;
    let names = service::get_names(id, &state).await?;
    Ok(Json(LeaveRequestDetailsResponse {
        request: leave_request.into(),
        employee_name: names.employee_name,
        leave_type: names.leave_type,
    }))
}

pub async fn create_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<LeaveRequestCreate>,
) -> Result<(StatusCode, Json<LeaveRequestResponse>), AppError> {
    let target = employees::service::get_access_by_id(input.employee_unid, &state).await?;
    authorize_on(&user, &target, LeaveRequestPermission::Create)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_request = service::create(input, user.unid, &state).await?;
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
    authorize_on(&user, &target, LeaveRequestPermission::Update)?;
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
        Pending => authorize_on(&user, &target, LeaveRequestPermission::Update)?,
        Accepted => authorize_approve(&user, &target)?,
        _ => authorize_reject(&user, &target)?,
    }

    let leave_request = service::update_leave_request_status(id, input, user.unid, &state).await?;
    Ok(Json(leave_request.into()))
}
