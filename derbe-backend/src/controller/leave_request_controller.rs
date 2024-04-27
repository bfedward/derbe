use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};

use crate::{error::AppError, http::AppContext, model::leave_request::{self, LeaveRequest}, service::leave_request_service};



pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveRequests/:id", get(get_leave_request))
        .route("/leaveRequests", post(create_leave_request))
}

async fn get_leave_request(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<LeaveRequest>, AppError> {
    match leave_request_service::get_leave_request(id, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request)),
        Err(e) => Err(e)
    }
}

async fn create_leave_request(State(ctx): State<AppContext>, Json(leave_request): Json<LeaveRequest>) -> Result<Json<LeaveRequest>, AppError> {
    match leave_request_service::create_leave_request(&ctx, leave_request).await {
        Ok(leave_request) => Ok(Json(leave_request)),
        Err(e) => Err(e)
    }
}