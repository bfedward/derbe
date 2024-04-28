use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};

use crate::{error::AppError, http::AppContext, model::leave_request::LeaveRequest, service::{leave_request_service::LeaveRequestService, DerbeService}};



pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveRequests/:id", get(get_leave_request))
        .route("/leaveRequests", post(create_leave_request))
}

async fn get_leave_request(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<LeaveRequest>, AppError> {
    match LeaveRequestService::get_entity_by_id(id, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request)),
        Err(e) => Err(e)
    }
}

async fn create_leave_request(State(ctx): State<AppContext>, Json(leave_request): Json<LeaveRequest>) -> Result<Json<LeaveRequest>, AppError> {
    match LeaveRequestService::create_entity(leave_request, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request)),
        Err(e) => Err(e)
    }
}