use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::AppError,
    http::AppContext,
    model::leave_type::LeaveType,
    service::{leave_types_service::LeaveTypesService, DerbeService},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveTypes", get(get_leave_types))
        .route("/leaveTypes/:id", get(get_leave_type))
}

async fn get_leave_types(State(ctx): State<AppContext>) -> Result<Json<Vec<LeaveType>>, AppError> {
    match LeaveTypesService::get_entities(&ctx).await {
        Ok(leave_types) => Ok(Json(leave_types)),
        Err(e) => Err(e),
    }
}

async fn get_leave_type(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<LeaveType>, AppError> {
    match LeaveTypesService::get_entity_by_id(id, &ctx).await {
        Ok(leave_type) => Ok(Json(leave_type)),
        Err(e) => Err(e),
    }
}
