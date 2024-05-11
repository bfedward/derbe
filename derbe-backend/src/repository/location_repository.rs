use crate::{error::AppError, model::location::Location};

use super::DerbeRepository;

pub struct LocationRepository {}

impl DerbeRepository<Location> for LocationRepository {
    async fn get_entity_by_id(
        id: i32,
        ctx: &crate::http::AppContext,
    ) -> Result<Location, crate::error::AppError> {
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

    async fn get_entities(
        ctx: &crate::http::AppContext,
    ) -> Result<Vec<Location>, crate::error::AppError> {
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

    async fn create_entity(
        new_entity: Location,
        ctx: &crate::http::AppContext,
    ) -> Result<Location, crate::error::AppError> {
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
        ctx: &crate::http::AppContext,
    ) -> Result<Location, crate::error::AppError> {
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

    async fn delete_entity(
        id: i32,
        ctx: &crate::http::AppContext,
    ) -> Result<(), crate::error::AppError> {
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
}
