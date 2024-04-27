use crate::{
    error::AppError, http::AppContext, model::leave_type::LeaveType, repository::leave_types_repository,
};

pub async fn get_leave_types(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
    leave_types_repository::get_leave_types(ctx).await
}