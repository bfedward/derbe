use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType};

pub async fn get_leave_type_by_id(id: i32, ctx: &AppContext) -> Result<LeaveType, AppError> {
    match sqlx::query_as!(
        LeaveType,
        r#"
        SELECT id, name, default_quota, can_book_in_future, 
            show_in_others_days, default_in_others_days, show_in_approve_days
        FROM leave_types
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&ctx.db)
    .await
    {
        Ok(Some(leave_type)) => Ok(leave_type),
        Ok(None) => Err(AppError::EntityNotFound(id, "Leave Type".to_owned())),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

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