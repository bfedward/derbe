use crate::{error::AppError, http::AppContext, model::leave_type::LeaveType};

use super::DerbeRepository;

pub struct LeaveTypesRepository {}

impl DerbeRepository<LeaveType> for LeaveTypesRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveType, AppError> {
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

    async fn get_entities(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
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

    async fn create_entity(new_entity: LeaveType, ctx: &AppContext) -> Result<LeaveType, AppError> {
        todo!()
    }

    async fn update_entity(
        entity_update: LeaveType,
        ctx: &AppContext,
    ) -> Result<LeaveType, AppError> {
        todo!()
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }
}
