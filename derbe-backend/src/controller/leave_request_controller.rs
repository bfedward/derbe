use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use derbe_shared::dto::{
    request::leave_request::{
        AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
        MyLeaveRequest,
    },
    response::leave_request::{AllLeaveResponse, LeaveRequestResponse, MyLeaveResponse},
};

use crate::{error::AppError, http::AppContext, service::leave_request_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveRequests/{id}", get(get_leave_request))
        .route("/leaveRequests", post(create_leave_request))
        .route("/myLeave", post(get_my_leave))
        .route("/allLeave", post(get_all_leave_table_data))
        .route("/leaveRequests/{id}", put(update_leave_request))
        .route(
            "/leaveRequests/{id}/status",
            put(update_leave_request_status),
        )
}

async fn get_my_leave(
    State(ctx): State<AppContext>,
    Json(my_leave_request): Json<MyLeaveRequest>,
) -> Result<Json<Vec<MyLeaveResponse>>, AppError> {
    match leave_request_service::get_my_leave(my_leave_request, &ctx).await {
        Ok(leave_requests) => Ok(Json(leave_requests)),
        Err(e) => Err(e),
    }
}

async fn get_all_leave_table_data(
    State(ctx): State<AppContext>,
    Json(all_leave_request): Json<AllLeaveRequest>,
) -> Result<Json<Vec<AllLeaveResponse>>, AppError> {
    match leave_request_service::get_all_leave_table_data(all_leave_request, &ctx).await {
        Ok(all_leave) => Ok(Json(all_leave)),
        Err(e) => Err(e),
    }
}

async fn get_leave_request(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    match leave_request_service::get_entity_by_id(id, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request.into())),
        Err(e) => Err(e),
    }
}

async fn create_leave_request(
    State(ctx): State<AppContext>,
    Json(leave_request): Json<LeaveRequestCreate>,
) -> Result<Response, AppError> {
    match leave_request_service::create_entity(leave_request, &ctx).await {
        Ok(leave_request) => {
            let leave_request: LeaveRequestResponse = leave_request.into();
            let response = (StatusCode::CREATED, Json(leave_request)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

async fn update_leave_request(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(leave_request): Json<LeaveRequestUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    match leave_request_service::update_entity(id, leave_request, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request.into())),
        Err(e) => Err(e),
    }
}

async fn update_leave_request_status(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(state_request): Json<LeaveRequestStatusUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    match leave_request_service::update_leave_request_status(id, state_request, &ctx).await {
        Ok(leave_request) => Ok(Json(leave_request.into())),
        Err(e) => Err(e),
    }
}
