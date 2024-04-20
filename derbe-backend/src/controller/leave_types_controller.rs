use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};

use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType, service::leave_types_service};


pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/leaveTypes", get(get_leave_types))
        // .route("/drivers", post(create_driver))
        // .route("/drivers/:id", get(get_driver))
        // .route("/drivers/:id", delete(delete_driver))
}

async fn get_leave_types(State(ctx): State<AppContext>) -> Result<Json<Vec<LeaveType>>, AppError> {
    match leave_types_service::get_leave_types(&ctx).await {
        Ok(leave_types) => Ok(Json(leave_types)),
        Err(e) => Err(e),
    }
}