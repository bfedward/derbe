use crate::features::divisions::dto::request::{DivisionCreate, DivisionUpdate};

use crate::{error::AppError, features::divisions::model::Division, http::AppState};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<Division, AppError> {
    sqlx::query_as!(
        Division,
        r#"
            SELECT id, name
            FROM divisions
            WHERE id = $1
            "#,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "Division".to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<Division>, AppError> {
    Ok(sqlx::query_as!(
        Division,
        r#"
            SELECT id, name
            FROM divisions
            "#,
    )
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn create(create_dto: DivisionCreate, ctx: &AppState) -> Result<Division, AppError> {
    Ok(sqlx::query_as!(
        Division,
        r#"
            INSERT INTO divisions (name)
            VALUES ($1)
            RETURNING id, name
            "#,
        create_dto.name,
    )
    .fetch_one(&ctx.pool)
    .await?)
}

pub async fn update(id: i32, dto: DivisionUpdate, ctx: &AppState) -> Result<Division, AppError> {
    sqlx::query_as!(
        Division,
        r#"
            UPDATE divisions
            SET name = $1
            WHERE id = $2
            RETURNING id,
                name
            "#,
        dto.name,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "Division".to_owned()))
}

pub async fn delete(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM divisions WHERE id = $1 RETURNING id", id)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM divisions WHERE id = $1)", id)
        .fetch_one(&ctx.pool)
        .await?
        .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Division".to_owned()))
    }
}

pub async fn ensure_not_referenced(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM employees WHERE division_id = $1
            ",
        id
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::RelatedEntity("Division".to_owned()))
    } else {
        Ok(())
    }
}

pub async fn ensure_name_available(name: &str, ctx: &AppState) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM divisions WHERE UPPER(name) = $1
        ",
        name.to_uppercase()
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::DuplicateNamedEntity("Division".to_owned()))
    } else {
        Ok(())
    }
}
