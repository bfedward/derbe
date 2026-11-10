use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::AppError,
    features::leave_types::dto::{request::LeaveTypeCreate, response::LeaveTypeResponse},
    features::leave_types::service,
    http::AppState,
};

use validator::Validate;

pub async fn list_leave_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<LeaveTypeResponse>>, AppError> {
    let leave_types = service::list(&state).await?;
    Ok(Json(
        leave_types
            .into_iter()
            .map(LeaveTypeResponse::from)
            .collect(),
    ))
}

pub async fn get_leave_type(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveTypeResponse>, AppError> {
    let leave_type = service::get_by_id(id, &state).await?;
    Ok(Json(leave_type.into()))
}

pub async fn create_leave_type(
    State(state): State<AppState>,
    Json(input): Json<LeaveTypeCreate>,
) -> Result<(StatusCode, Json<LeaveTypeResponse>), AppError> {
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let leave_type = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(leave_type.into())))
}
