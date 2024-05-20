use crate::{error::AppError, http::AppContext, model::division::Division};

use super::DerbeRepository;

pub struct DivisionRepository {}

impl DerbeRepository<Division> for DivisionRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Division, AppError> {
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

    async fn get_entities(ctx: &AppContext) -> Result<Vec<Division>, AppError> {
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

    async fn create_entity(new_entity: Division, ctx: &AppContext) -> Result<Division, AppError> {
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
        ctx: &AppContext,
    ) -> Result<Division, AppError> {
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

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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

    async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM divisions WHERE id = $1)", id)
                .fetch_one(&ctx.db)
                .await
                .map_err(AppError::DatabaseQueryError)?;

        if exists.unwrap_or(false) {
            Ok(())
        } else {
            Err(AppError::EntityNotFound(id, "Division".to_owned()))
        }
    }

    async fn check_if_entity_is_referenced(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        let count = sqlx::query_scalar!(
            "
            SELECT COUNT(*) FROM employees WHERE division_id = $1
            ",
            id
        )
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?
        .unwrap_or(0);

        if count > 0 {
            Err(AppError::RelatedEntity("Division".to_owned()))
        } else {
            Ok(())
        }
    }
}
