use crate::{error::AppError, http::AppContext, model::location::Location};

use super::DerbeRepository;

pub struct LocationRepository {}

impl DerbeRepository<Location> for LocationRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Location, AppError> {
        match sqlx::query_as!(
            Location,
            r#"
            SELECT id, name
            FROM locations
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(Some(location)) => Ok(location),
            Ok(None) => Err(AppError::EntityNotFound(id, "Location".to_owned())),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn get_entities(ctx: &AppContext) -> Result<Vec<Location>, AppError> {
        match sqlx::query_as!(
            Location,
            r#"
            SELECT id, name 
            FROM locations
            "#,
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(locations) => Ok(locations),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn create_entity(new_entity: Location, ctx: &AppContext) -> Result<Location, AppError> {
        match sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            new_entity.name,
        )
        .fetch_one(&ctx.db)
        .await
        {
            Ok(location) => Ok(location),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn update_entity(
        id: i32,
        entity_update: Location,
        ctx: &AppContext,
    ) -> Result<Location, AppError> {
        match sqlx::query_as!(
            Location,
            r#"
            UPDATE locations
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
            Ok(location) => match location {
                Some(location) => Ok(location),
                None => Err(AppError::EntityNotFound(id, "Location".to_owned())),
            },
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        match sqlx::query!(
            r#"DELETE FROM locations
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
        match sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM locations WHERE id = $1)", id)
            .fetch_one(&ctx.db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }
}
