use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType};

pub async fn get_leave_types(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
    match sqlx::query_as!(
        LeaveType,
        r#"
        SELECT id, name, default_quota, can_book_in_future, 
            show_in_others_days, default_in_others_days, show_in_approve_days
        FROM leave_types
        "#,
    )
    .fetch_all(&ctx.db)
    .await
    {
        Ok(leave_types) => Ok(leave_types),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}