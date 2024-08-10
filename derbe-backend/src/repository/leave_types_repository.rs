use derbe_shared::dto::request::leave_type::{LeaveTypeCreate, LeaveTypeUpdate};

use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveType, AppError> {
    match sqlx::query_as!(
            LeaveType,
            r#"
            SELECT id, name, default_quota, colour, can_book_in_future, 
                show_in_others_days, default_in_others_days, default_in_leave_request, show_in_approve_days
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

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
    match sqlx::query_as!(
            LeaveType,
            r#"
            SELECT id, name, default_quota, colour, can_book_in_future, 
                show_in_others_days, default_in_others_days, default_in_leave_request, show_in_approve_days
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

pub async fn create_entity(dto: LeaveTypeCreate, ctx: &AppContext) -> Result<LeaveType, AppError> {
    match sqlx::query_as!(
        LeaveType,
        r#"
            INSERT INTO leave_types (name, 
                                    default_quota, 
                                    colour,
                                    can_book_in_future, 
                                    show_in_others_days, 
                                    default_in_others_days, 
                                    default_in_leave_request, 
                                    show_in_approve_days)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, 
                        name, 
                        default_quota,
                        colour,
                        can_book_in_future, 
                        show_in_others_days, 
                        default_in_others_days, 
                        default_in_leave_request, 
                        show_in_approve_days
            "#,
        dto.name,
        dto.default_quota,
        dto.colour,
        dto.can_book_in_future,
        dto.show_in_others_days,
        dto.default_in_others_days,
        dto.default_in_leave_request,
        dto.show_in_approve_days
    )
    .fetch_one(&ctx.db)
    .await
    {
        Ok(leave_type) => Ok(leave_type),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

pub async fn update_entity(
    id: i32,
    dto: LeaveTypeUpdate,
    ctx: &AppContext,
) -> Result<LeaveType, AppError> {
    match sqlx::query_as!(
        LeaveType,
        r#"
            UPDATE leave_types
            SET name = $1,
                default_quota = $2,
                colour = $3,
                can_book_in_future = $4,
                show_in_others_days = $5,
                default_in_others_days = $6,
                default_in_leave_request = $7,
                show_in_approve_days = $8
            WHERE id = $9
            RETURNING id, 
                name, 
                default_quota, 
                colour,
                can_book_in_future, 
                show_in_others_days, 
                default_in_others_days, 
                default_in_leave_request, 
                show_in_approve_days
            "#,
        dto.name,
        dto.default_quota,
        dto.colour,
        dto.can_book_in_future,
        dto.show_in_others_days,
        dto.default_in_others_days,
        dto.default_in_leave_request,
        dto.show_in_approve_days,
        id
    )
    .fetch_optional(&ctx.db)
    .await
    {
        Ok(leave_type) => match leave_type {
            Some(leave_type) => Ok(leave_type),
            None => Err(AppError::EntityNotFound(id, "Leave Type".to_owned())),
        },
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

pub async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    match sqlx::query!(
        r#"DELETE FROM leave_types
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

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM leave_types WHERE id = $1)", id)
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

    if exists.unwrap_or(false) {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Leave Type".to_owned()))
    }
}

pub async fn check_if_entity_is_referenced(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM leave_requests WHERE leave_type_id = $1
            ",
        id
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(AppError::DatabaseQueryError)?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::RelatedEntity("Leave Type".to_owned()))
    } else {
        Ok(())
    }
}
