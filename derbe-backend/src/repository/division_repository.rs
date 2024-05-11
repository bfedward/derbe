use crate::{
    error::AppError,
    model::division::Division,
};

use super::DerbeRepository;

pub struct DivisionRepository {}

impl DerbeRepository<Division> for DivisionRepository {
    async fn get_entity_by_id(
        id: i32,
        ctx: &crate::http::AppContext,
    ) -> Result<Division, crate::error::AppError> {
        match sqlx::query_as!(
            Division,
            r#"
            SELECT id, name
            FROM divisions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(Some(division)) => Ok(division),
            Ok(None) => Err(AppError::EntityNotFound(id, "Division".to_owned())),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn get_entities(
        ctx: &crate::http::AppContext,
    ) -> Result<Vec<Division>, crate::error::AppError> {
        match sqlx::query_as!(
            Division,
            r#"
            SELECT id, name 
            FROM divisions
            "#,
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(divisions) => Ok(divisions),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn create_entity(
        new_entity: Division,
        ctx: &crate::http::AppContext,
    ) -> Result<Division, crate::error::AppError> {
        match sqlx::query_as!(
            Division,
            r#"
            INSERT INTO divisions (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            new_entity.name,
        )
        .fetch_one(&ctx.db)
        .await
        {
            Ok(division) => Ok(division),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn update_entity(
        id: i32,
        entity_update: Division,
        ctx: &crate::http::AppContext,
    ) -> Result<Division, crate::error::AppError> {
        match sqlx::query_as!(
            Division,
            r#"
            UPDATE divisions
            SET name = $1
            WHERE id = $2
            RETURNING id, 
                name
            "#,
            entity_update.name,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(division) => match division {
                Some(division) => Ok(division),
                None => Err(AppError::EntityNotFound(id, "Division".to_owned())),
            },
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn delete_entity(
        id: i32,
        ctx: &crate::http::AppContext,
    ) -> Result<(), crate::error::AppError> {
        match sqlx::query!(
            r#"DELETE FROM divisions
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
}
