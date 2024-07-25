use std::collections::HashMap;

use chrono::NaiveDate;
use derbe_shared::dto::request::leave_request::AllLeaveRequest;
use derbe_shared::dto::response::leave_request::{AllLeaveInnerResponse, AllLeaveResponse};

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
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM leave_requests WHERE id = $1)",
            id
        )
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

        if exists.unwrap_or(false) {
            Ok(())
        } else {
            Err(AppError::EntityNotFound(id, "Leave Request".to_owned()))
        }
    }

    async fn check_if_entity_is_referenced(_id: i32, _ctx: &AppContext) -> Result<(), AppError> {
        Ok(())
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
            ORDER BY lr.date_from DESC
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

    pub async fn check_for_accepted_leave_for_employee_and_date_range(
        employee_id: i32,
        date_from: NaiveDate,
        date_to: NaiveDate,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 
                FROM leave_requests 
                WHERE employee_id = $1 
                AND leave_status = $2
                AND date_from BETWEEN $3 AND $4 
                AND date_to BETWEEN $3 AND $4)",
            employee_id,
            LeaveStatus::Accepted as LeaveStatus,
            date_from,
            date_to
        )
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

        if !exists.unwrap_or(false) {
            Ok(())
        } else {
            Err(AppError::ValidationFailed(
                "Employee has an Accepted leave request within this date range".to_owned(),
            ))
        }
    }

    pub async fn get_all_leave_table_data(
        all_leave_request: AllLeaveRequest,
        ctx: &AppContext,
    ) -> Result<Vec<AllLeaveResponse>, AppError> {
        let leave_requests = sqlx::query!(
            r#"
            SELECT 
                CONCAT(e.first_name, ' ', e.last_name) AS employee_name, 
                lr.date_from, 
                lr.date_to, 
                lr.first_day_half, 
                lr.last_day_half,
                lt.colour
            FROM leave_requests lr
            JOIN employees e ON lr.employee_id = e.id
            JOIN leave_types lt ON lr.leave_type_id = lt.id
            WHERE lr.leave_status = 'Accepted'
            AND lr.date_from >= $1
            AND lr.date_to <= $2
            ORDER BY e.last_name, e.first_name, lr.date_from DESC
            "#,
            all_leave_request.date_from,
            all_leave_request.date_to
        )
        .fetch_all(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

        let mut employee_map: HashMap<String, Vec<AllLeaveInnerResponse>> = HashMap::new();

        for row in leave_requests {
            if let Some(employee_name) = row.employee_name {
                let entry = employee_map.entry(employee_name).or_default();
                entry.push(AllLeaveInnerResponse {
                    date_from: row.date_from,
                    date_to: row.date_to,
                    first_day_half: row.first_day_half,
                    last_day_half: row.last_day_half,
                    colour: row.colour,
                });
            }
        }

        let all_leave_responses: Vec<AllLeaveResponse> = employee_map
            .into_iter()
            .map(|(employee_name, leaves)| AllLeaveResponse {
                employee_name,
                leaves,
            })
            .collect();

        Ok(all_leave_responses)
    }
}
