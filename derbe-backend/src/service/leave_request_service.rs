use crate::{error::AppError, http::AppContext, model::leave_request::LeaveRequest, repository::leave_request_respository};



pub async fn get_leave_request(id: i32, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
    leave_request_respository::get_leave_request_by_id(id, ctx).await
}

pub async fn create_leave_request(ctx: &AppContext, new_leave_request: LeaveRequest) -> Result<LeaveRequest, AppError> {
    // TODO: check all the related entities!
    leave_request_respository::create_leave_request(ctx, new_leave_request).await
}