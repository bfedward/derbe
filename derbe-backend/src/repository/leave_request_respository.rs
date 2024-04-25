use crate::{error::AppError, http::AppContext, model::leave_request::{self, LeaveRequest, LeaveStatus}};
use crate::model;

pub async fn create_leave_request(ctx: &AppContext, new_leave_request: LeaveRequest) -> Result<LeaveRequest, AppError> {
    match sqlx::query_as!(
        LeaveRequest,
        r#"
        INSERT INTO leave_requests (employee_id, 
                                    leave_type_id, 
                                    leave_status,
                                    date_from, 
                                    date_to,
                                    first_day_half, 
                                    last_day_half, 
                                    date_requested,
                                    date_finalised,
                                    notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, 
                    employee_id, 
                    leave_type_id, 
                    leave_status AS "leave_status!: model::leave_request::LeaveStatus", 
                    date_from, 
                    date_to,
                    first_day_half, 
                    last_day_half, 
                    date_requested,
                    date_finalised,
                    notes
        "#,
        new_leave_request.employee_id,
        new_leave_request.leave_type_id,
        new_leave_request.leave_status as LeaveStatus,
        new_leave_request.date_from,
        new_leave_request.date_to,
        new_leave_request.first_day_half,
        new_leave_request.last_day_half,
        new_leave_request.date_requested,
        new_leave_request.date_finalised,
        new_leave_request.notes
    )
    .fetch_one(&ctx.db)
    .await
    {
        Ok(leave_request) => Ok(leave_request),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

pub async fn get_leave_request_by_id(id: i32, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
    match sqlx::query_as!(
       LeaveRequest,
        r#"
        SELECT id, 
                employee_id, 
                leave_type_id, 
                leave_status AS "leave_status!: model::leave_request::LeaveStatus", 
                date_from,
                date_to,
                first_day_half, 
                last_day_half, 
                date_requested,
                date_finalised,
                notes
        FROM leave_requests
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(&ctx.db)
    .await
    {
        Ok(Some(leave_request)) => Ok(leave_request),
        Ok(None) => Err(AppError::EntityNotFound(id, "Leave Request".to_owned())),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}