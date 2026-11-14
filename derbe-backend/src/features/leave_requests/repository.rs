use std::collections::HashMap;
use std::str::FromStr;

use crate::enums::LeaveStatus;
use crate::features::leave_requests::dto::request::{
    AllLeaveRequest, LeaveRequestCreate, LeaveRequestUpdate, MyLeaveRequest,
};
use crate::features::leave_requests::dto::response::{
    AllLeaveInnerResponse, AllLeaveResponse, MyLeaveResponse,
};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{error::AppError, features::leave_requests::model::LeaveRequest, http::AppState};

struct AllLeaveRow {
    employee_name: Option<String>,
    date_from: NaiveDate,
    date_to: NaiveDate,
    first_day_half: bool,
    last_day_half: bool,
    colour: String,
}

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LeaveRequest, AppError> {
    sqlx::query_as!(
        LeaveRequest,
        r#"
             SELECT id, 
                     employee_unid, 
                     leave_type_id, 
                     leave_status AS "leave_status!: LeaveStatus", 
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
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "Leave Request".to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<LeaveRequest>, AppError> {
    Ok(sqlx::query_as!(
        LeaveRequest,
        r#"
            SELECT id, 
                employee_unid, 
                leave_type_id, 
                leave_status AS "leave_status!: LeaveStatus", 
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
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn create(dto: LeaveRequestCreate, ctx: &AppState) -> Result<LeaveRequest, AppError> {
    Ok(sqlx::query_as!(
        LeaveRequest,
        r#"
            INSERT INTO leave_requests (employee_unid, 
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
                        employee_unid, 
                        leave_type_id, 
                        leave_status AS "leave_status!: LeaveStatus", 
                        date_from, 
                        date_to,
                        first_day_half, 
                        last_day_half, 
                        total_duration,
                        date_requested,
                        date_finalised,
                        notes
            "#,
        dto.employee_unid,
        dto.leave_type_id,
        dto.leave_status as LeaveStatus,
        dto.values.date_from,
        dto.values.date_to,
        dto.values.first_day_half,
        dto.values.last_day_half,
        dto.values.total_duration,
        dto.date_requested,
        dto.date_finalised,
        dto.notes
    )
    .fetch_one(&ctx.pool)
    .await?)
}

pub async fn update(
    id: i32,
    dto: LeaveRequestUpdate,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    sqlx::query_as!(
        LeaveRequest,
        r#"
            UPDATE leave_requests
            SET employee_unid = $1,
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
                employee_unid, 
                leave_type_id, 
                leave_status AS "leave_status!: LeaveStatus", 
                date_from, 
                date_to,
                first_day_half, 
                last_day_half, 
                total_duration,
                date_requested,
                date_finalised,
                notes
            "#,
        dto.employee_unid,
        dto.leave_type_id,
        dto.leave_status as LeaveStatus,
        dto.values.date_from,
        dto.values.date_to,
        dto.values.first_day_half,
        dto.values.last_day_half,
        dto.values.total_duration,
        dto.date_requested,
        dto.date_finalised,
        dto.notes,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "LeaveRequest".to_owned()))
}

pub async fn delete(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM leave_requests WHERE id = $1 RETURNING id", id)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM leave_requests WHERE id = $1)",
        id
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Leave Request".to_owned()))
    }
}

pub async fn ensure_not_referenced(_id: i32, _ctx: &AppState) -> Result<(), AppError> {
    Ok(())
}

pub async fn get_my_leave(
    dto: MyLeaveRequest,
    ctx: &AppState,
) -> Result<Vec<MyLeaveResponse>, AppError> {
    Ok(sqlx::query_as!(
        MyLeaveResponse,
        r#"
            SELECT
                lr.id AS "leave_request_id", 
                lt.name AS "leave_type",
                lr.leave_status AS "leave_status!: LeaveStatus", 
                lr.date_from,
                lr.date_to,
                lr.total_duration
            FROM
                leave_requests lr
            JOIN
                leave_types lt ON lr.leave_type_id = lt.id
            WHERE
                employee_unid = $1
                AND leave_type_id = ANY($2)
                AND leave_status = ANY($3)
                AND date_from BETWEEN $4 AND $5 
                AND date_to BETWEEN $4 AND $5
            ORDER BY lr.date_from DESC
            "#,
        dto.employee_unid,
        &dto.leave_type_ids,
        dto.leave_statuses as Vec<LeaveStatus>,
        dto.date_from,
        dto.date_to
    )
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn ensure_no_accepted_leave_conflict(
    employee_unid: Uuid,
    date_from: NaiveDate,
    date_to: NaiveDate,
    ctx: &AppState,
) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 
                FROM leave_requests 
                WHERE employee_unid = $1 
                AND leave_status = $2
                AND date_from BETWEEN $3 AND $4 
                AND date_to BETWEEN $3 AND $4)",
        employee_unid,
        LeaveStatus::Accepted as LeaveStatus,
        date_from,
        date_to
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(false);

    if !exists {
        Ok(())
    } else {
        Err(AppError::ValidationFailed(
            "Employee has an Accepted leave request within this date range".to_owned(),
        ))
    }
}

pub async fn get_all_leave(
    all_leave_request: AllLeaveRequest,
    ctx: &AppState,
) -> Result<Vec<AllLeaveResponse>, AppError> {
    let leave_requests = sqlx::query_as!(
        AllLeaveRow,
        r#"
            SELECT 
                CONCAT(e.first_name, ' ', e.last_name) AS employee_name, 
                lr.date_from, 
                lr.date_to, 
                lr.first_day_half, 
                lr.last_day_half,
                lt.colour
            FROM leave_requests lr
            JOIN employees e ON lr.employee_unid = e.unid
            JOIN leave_types lt ON lr.leave_type_id = lt.id
            WHERE lr.leave_status = 'Accepted'
            AND lr.date_from >= $1
            AND lr.date_to <= $2
            ORDER BY e.last_name, e.first_name, lr.date_from DESC
            "#,
        all_leave_request.date_from,
        all_leave_request.date_to
    )
    .fetch_all(&ctx.pool)
    .await?;

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
