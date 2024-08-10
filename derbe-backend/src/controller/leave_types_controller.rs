use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use derbe_shared::dto::{
    request::leave_type::LeaveTypeCreate, response::leave_type::LeaveTypeResponse,
};

use crate::{error::AppError, http::AppContext, service::leave_types_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveTypes", get(get_leave_types))
        .route("/leaveTypes/:id", get(get_leave_type))
        .route("/leaveTypes", post(create_leave_type))
}

async fn get_leave_types(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<LeaveTypeResponse>>, AppError> {
    match leave_types_service::get_entities(&ctx).await {
        Ok(leave_types) => {
            let reps = leave_types
                .into_iter()
                .map(LeaveTypeResponse::from)
                .collect();
            Ok(Json(reps))
        }
        Err(e) => Err(e),
    }
}

async fn get_leave_type(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<LeaveTypeResponse>, AppError> {
    match leave_types_service::get_entity_by_id(id, &ctx).await {
        Ok(leave_type) => {
            let res = leave_type.into();
            Ok(Json(res))
        }
        Err(e) => Err(e),
    }
}

async fn create_leave_type(
    State(ctx): State<AppContext>,
    Json(leave_type): Json<LeaveTypeCreate>,
) -> Result<Response, AppError> {
    match leave_types_service::create_entity(leave_type, &ctx).await {
        Ok(leave_type) => {
            let leave_type: LeaveTypeResponse = leave_type.into();
            let response = (StatusCode::CREATED, Json(leave_type)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
