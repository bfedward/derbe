use crate::dto::request::{LeaveTypeCreate, LeaveTypeUpdate};
use crate::model::OthersDaysVisibility;
use derbe_core::error::AppError;
use derbe_core::state::AppState;

use crate::dto::response::LeaveTypeResponse;

const ENTITY: &str = "Leave Type";

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LeaveTypeResponse, AppError> {
    sqlx::query_as!(
            LeaveTypeResponse,
            r#"
            SELECT id, name, default_quota, colour, can_book_in_future, 
                others_days_visibility AS "others_days_visibility: OthersDaysVisibility", default_in_leave_request, show_in_approve_days
            FROM leave_types
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&ctx.pool)
        .await?.ok_or_else(|| AppError::EntityNotFound(id, ENTITY.to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<LeaveTypeResponse>, AppError> {
    Ok(sqlx::query_as!(
            LeaveTypeResponse,
            r#"
            SELECT id, name, default_quota, colour, can_book_in_future, 
                others_days_visibility AS "others_days_visibility: OthersDaysVisibility", default_in_leave_request, show_in_approve_days
            FROM leave_types
            "#,
        )
        .fetch_all(&ctx.pool)
        .await?)
}

pub async fn create(dto: LeaveTypeCreate, ctx: &AppState) -> Result<LeaveTypeResponse, AppError> {
    Ok(sqlx::query_as!(
        LeaveTypeResponse,
        r#"
            INSERT INTO leave_types (name, 
                                    default_quota, 
                                    colour,
                                    can_book_in_future, 
                                    others_days_visibility, 
                                    default_in_leave_request, 
                                    show_in_approve_days)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, 
                        name, 
                        default_quota,
                        colour,
                        can_book_in_future, 
                        others_days_visibility AS "others_days_visibility: OthersDaysVisibility", 
                        default_in_leave_request, 
                        show_in_approve_days
            "#,
        dto.name,
        dto.default_quota,
        dto.colour,
        dto.can_book_in_future,
        dto.others_days_visibility as OthersDaysVisibility,
        dto.default_in_leave_request,
        dto.show_in_approve_days
    )
    .fetch_one(&ctx.pool)
    .await
    .map_err(|error| AppError::duplicate_name_or_db(error, ENTITY))?)
}

pub async fn update(
    id: i32,
    dto: LeaveTypeUpdate,
    ctx: &AppState,
) -> Result<LeaveTypeResponse, AppError> {
    sqlx::query_as!(
        LeaveTypeResponse,
        r#"
            UPDATE leave_types
            SET name = $1,
                default_quota = $2,
                colour = $3,
                can_book_in_future = $4,
                others_days_visibility = $5,
                default_in_leave_request = $6,
                show_in_approve_days = $7
            WHERE id = $8
            RETURNING id, 
                name, 
                default_quota, 
                colour,
                can_book_in_future, 
                others_days_visibility AS "others_days_visibility: OthersDaysVisibility", 
                default_in_leave_request, 
                show_in_approve_days
            "#,
        dto.name,
        dto.default_quota,
        dto.colour,
        dto.can_book_in_future,
        dto.others_days_visibility as OthersDaysVisibility,
        dto.default_in_leave_request,
        dto.show_in_approve_days,
        id
    )
    .fetch_optional(&ctx.pool)
    .await
    .map_err(|error| AppError::duplicate_name_or_db(error, ENTITY))?
    .ok_or_else(|| AppError::EntityNotFound(id, ENTITY.to_owned()))
}

pub async fn delete(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM leave_types WHERE id = $1 RETURNING id", id)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM leave_types WHERE id = $1)", id)
        .fetch_one(&ctx.pool)
        .await?
        .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, ENTITY.to_owned()))
    }
}

pub async fn ensure_not_referenced(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM leave_requests WHERE leave_type_id = $1
            ",
        id
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::RelatedEntity(ENTITY.to_owned()))
    } else {
        Ok(())
    }
}
