use crate::model;
use crate::{
    dto::{request::leave_request::MyLeaveRequest, response::leave_request::MyLeaveResponse},
    error::AppError,
    http::AppContext,
    model::leave_request::{LeaveRequest, LeaveStatus},
};

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
                     total_duration,
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
                total_duration,
                date_requested,
                date_finalised,
                notes
            FROM leave_requests
            "#
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(vehicles) => Ok(vehicles),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn create_entity(
        new_entity: LeaveRequest,
        ctx: &AppContext,
    ) -> Result<LeaveRequest, AppError> {
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
                                        total_duration,
                                        date_requested,
                                        date_finalised,
                                        notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, 
                        employee_id, 
                        leave_type_id, 
                        leave_status AS "leave_status!: model::leave_request::LeaveStatus", 
                        date_from, 
                        date_to,
                        first_day_half, 
                        last_day_half, 
                        total_duration,
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
            new_entity.total_duration,
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

    async fn update_entity(
        id: i32,
        entity_update: LeaveRequest,
        ctx: &AppContext,
    ) -> Result<LeaveRequest, AppError> {
        match sqlx::query_as!(
            LeaveRequest,
            r#"
            UPDATE leave_requests
            SET employee_id = $1,
                leave_type_id = $2,
                leave_status = $3,
                date_from = $4,
                date_to = $5,
                first_day_half = $6,
                last_day_half = $7,
                total_duration = $8,
                date_requested = $9,
                date_finalised = $10,
                notes = $11
            WHERE id = $12
            RETURNING id, 
                employee_id, 
                leave_type_id, 
                leave_status AS "leave_status!: model::leave_request::LeaveStatus", 
                date_from, 
                date_to,
                first_day_half, 
                last_day_half, 
                total_duration,
                date_requested,
                date_finalised,
                notes
            "#,
            entity_update.employee_id,
            entity_update.leave_type_id,
            entity_update.leave_status as LeaveStatus,
            entity_update.date_from,
            entity_update.date_to,
            entity_update.first_day_half,
            entity_update.last_day_half,
            entity_update.total_duration,
            entity_update.date_requested,
            entity_update.date_finalised,
            entity_update.notes,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(vehicle) => match vehicle {
                Some(vehicle) => Ok(vehicle),
                None => Err(AppError::EntityNotFound(id, "LeaveRequest".to_owned())),
            },
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        match sqlx::query!(
            r#"DELETE FROM leave_requests
            WHERE id = $1
            "#,
            id
        )
        .execute(&ctx.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM leave_requests WHERE id = $1)", id)
            .fetch_one(&ctx.db)
            .await
            .map_err(AppError::DatabaseQueryError)?;
    
        if exists.unwrap_or(false) {
            Ok(())
        } else {
            Err(AppError::EntityNotFound(id, "Leave Request".to_owned()))
        }
    }
}

impl LeaveRequestRepository {
    pub async fn get_my_leave(
        my_leave_request: MyLeaveRequest,
        ctx: &AppContext,
    ) -> Result<Vec<MyLeaveResponse>, AppError> {
        match sqlx::query_as!(
            MyLeaveResponse,
            r#"
            SELECT
                lr.id AS "leave_request_id", 
                lt.name AS "leave_type",
                lr.leave_status AS "leave_status!: model::leave_request::LeaveStatus", 
                lr.date_from,
                lr.date_to,
                lr.total_duration
            FROM
                leave_requests lr
            JOIN
                leave_types lt ON lr.leave_type_id = lt.id
            WHERE
                employee_id = $1
                AND leave_type_id = ANY($2)
                AND leave_status = ANY($3)
                AND date_from BETWEEN $4 AND $5 
                AND date_to BETWEEN $4 AND $5
            "#,
            my_leave_request.employee_id,
            &my_leave_request.leave_type_ids,
            my_leave_request.leave_statuses as Vec<LeaveStatus>,
            my_leave_request.date_from,
            my_leave_request.date_to
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(my_leaves) => Ok(my_leaves),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }
}
