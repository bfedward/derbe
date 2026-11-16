use crate::features::locations::dto::request::{LocationCreate, LocationUpdate};

use crate::{
    error::AppError, features::locations::dto::response::LocationResponse, http::AppState,
};

const ENTITY: &str = "Location";

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LocationResponse, AppError> {
    sqlx::query_as!(
        LocationResponse,
        r#"
            SELECT id, name
            FROM locations
            WHERE id = $1
            "#,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, ENTITY.to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<LocationResponse>, AppError> {
    Ok(sqlx::query_as!(
        LocationResponse,
        r#"
            SELECT id, name
            FROM locations
            "#,
    )
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn create(dto: LocationCreate, ctx: &AppState) -> Result<LocationResponse, AppError> {
    Ok(sqlx::query_as!(
        LocationResponse,
        r#"
            INSERT INTO locations (name)
            VALUES ($1)
            RETURNING id, name
            "#,
        dto.name,
    )
    .fetch_one(&ctx.pool)
    .await
    .map_err(|error| AppError::duplicate_name_or_db(error, ENTITY))?)
}

pub async fn update(
    id: i32,
    dto: LocationUpdate,
    ctx: &AppState,
) -> Result<LocationResponse, AppError> {
    sqlx::query_as!(
        LocationResponse,
        r#"
            UPDATE locations
            SET name = $1
            WHERE id = $2
            RETURNING id,
                name
            "#,
        dto.name,
        id
    )
    .fetch_optional(&ctx.pool)
    .await
    .map_err(|error| AppError::duplicate_name_or_db(error, ENTITY))?
    .ok_or_else(|| AppError::EntityNotFound(id, ENTITY.to_owned()))
}

pub async fn delete(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM locations WHERE id = $1 RETURNING id", id)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM locations WHERE id = $1)", id)
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
            SELECT COUNT(*) FROM employees WHERE location_id = $1
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
