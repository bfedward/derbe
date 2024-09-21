use derbe_shared::dto::request::location::{LocationCreate, LocationUpdate};

use crate::{error::AppError, http::AppContext, model::location::Location};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Location, AppError> {
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

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<Location>, AppError> {
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

pub async fn create_entity(dto: LocationCreate, ctx: &AppContext) -> Result<Location, AppError> {
    match sqlx::query_as!(
        Location,
        r#"
            INSERT INTO locations (name)
            VALUES ($1)
            RETURNING id, name
            "#,
        dto.name,
    )
    .fetch_one(&ctx.db)
    .await
    {
        Ok(location) => Ok(location),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

pub async fn update_entity(
    id: i32,
    dto: LocationUpdate,
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
        dto.name,
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

pub async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM locations WHERE id = $1)", id)
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

    if exists.unwrap_or(false) {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Location".to_owned()))
    }
}

pub async fn check_if_entity_is_referenced(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM employees WHERE location_id = $1
            ",
        id
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(AppError::DatabaseQueryError)?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::RelatedEntity("Location".to_owned()))
    } else {
        Ok(())
    }
}

pub async fn check_if_entity_exists_by_name(name: &str, ctx: &AppContext) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM locations WHERE UPPER(name) = $1
        ",
        name.to_uppercase()
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(AppError::DatabaseQueryError)?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::DuplicateNamedEntity("Location".to_owned()))
    } else {
        Ok(())
    }
}
