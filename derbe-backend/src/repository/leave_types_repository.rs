use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType};

use super::DerbeRepository;

pub struct LeaveTypesRepository {}

impl DerbeRepository<LeaveType> for LeaveTypesRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveType, AppError> {
        match sqlx::query_as!(
            LeaveType,
            r#"
            SELECT id, name, default_quota, can_book_in_future, 
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

    async fn get_entities(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
        match sqlx::query_as!(
            LeaveType,
            r#"
            SELECT id, name, default_quota, can_book_in_future, 
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

    async fn create_entity(new_entity: LeaveType, ctx: &AppContext) -> Result<LeaveType, AppError> {
        match sqlx::query_as!(
            LeaveType,
            r#"
            INSERT INTO leave_types (name, 
                                    default_quota, 
                                    can_book_in_future, 
                                    show_in_others_days, 
                                    default_in_others_days, 
                                    default_in_leave_request, 
                                    show_in_approve_days)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, 
                        name, 
                        default_quota, 
                        can_book_in_future, 
                        show_in_others_days, 
                        default_in_others_days, 
                        default_in_leave_request, 
                        show_in_approve_days
            "#,
            new_entity.name,
            new_entity.default_quota,
            new_entity.can_book_in_future,
            new_entity.show_in_others_days,
            new_entity.default_in_others_days,
            new_entity.default_in_leave_request,
            new_entity.show_in_approve_days
        )
        .fetch_one(&ctx.db)
        .await
        {
            Ok(leave_type) => Ok(leave_type),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn update_entity(
        id: i32,
        entity_update: LeaveType,
        ctx: &AppContext,
    ) -> Result<LeaveType, AppError> {
        match sqlx::query_as!(
            LeaveType,
            r#"
            UPDATE leave_types
            SET name = $1,
                default_quota = $2,
                can_book_in_future = $3,
                show_in_others_days = $4,
                default_in_others_days = $5,
                default_in_leave_request = $6,
                show_in_approve_days = $7
            WHERE id = $8
            RETURNING id, 
                name, 
                default_quota, 
                can_book_in_future, 
                show_in_others_days, 
                default_in_others_days, 
                default_in_leave_request, 
                show_in_approve_days
            "#,
            entity_update.name,
            entity_update.default_quota,
            entity_update.can_book_in_future,
            entity_update.show_in_others_days,
            entity_update.default_in_others_days,
            entity_update.default_in_leave_request,
            entity_update.show_in_approve_days,
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

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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

    async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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
}
