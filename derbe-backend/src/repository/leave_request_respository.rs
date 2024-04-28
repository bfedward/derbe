use crate::{error::AppError, http::AppContext, model::leave_request::{LeaveRequest, LeaveStatus}};
use crate::model;

use super::DerbeRepository;

pub struct LeaveRequestRepository {}

impl DerbeRepository<LeaveRequest> for LeaveRequestRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
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

    async fn get_entities(ctx: &AppContext) -> Result<Vec<LeaveRequest>, AppError> {
        todo!()
    }

    async fn create_entity(new_entity: LeaveRequest, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
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
            new_entity.employee_id,
            new_entity.leave_type_id,
            new_entity.leave_status as LeaveStatus,
            new_entity.date_from,
            new_entity.date_to,
            new_entity.first_day_half,
            new_entity.last_day_half,
            new_entity.date_requested,
            new_entity.date_finalised,
            new_entity.notes
        )
        .fetch_one(&ctx.db)
        .await
        {
            Ok(leave_request) => Ok(leave_request),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn update_entity(entity_update: LeaveRequest, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
        todo!()
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }
}