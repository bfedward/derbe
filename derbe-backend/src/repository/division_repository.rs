use derbe_shared::dto::request::division::{DivisionCreate, DivisionUpdate};

use crate::{error::AppError, http::AppContext, model::division::Division};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Division, AppError> {
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

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<Division>, AppError> {
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

pub async fn create_entity(
    create_dto: DivisionCreate,
    ctx: &AppContext,
) -> Result<Division, AppError> {
    match sqlx::query_as!(
        Division,
        r#"
            INSERT INTO divisions (name)
            VALUES ($1)
            RETURNING id, name
            "#,
        create_dto.name,
    )
    .fetch_one(&ctx.db)
    .await
    {
        Ok(division) => Ok(division),
        Err(e) => Err(AppError::DatabaseQueryError(e)),
    }
}

pub async fn update_entity(
    id: i32,
    dto: DivisionUpdate,
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
        dto.name,
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

pub async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM divisions WHERE id = $1)", id)
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?;

    if exists.unwrap_or(false) {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Division".to_owned()))
    }
}

pub async fn check_if_entity_is_referenced(id: i32, ctx: &AppContext) -> Result<(), AppError> {
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

pub async fn check_if_entity_exists_by_name(name: &str, ctx: &AppContext) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT COUNT(*) FROM divisions WHERE UPPER(name) = $1
        ",
        name.to_uppercase()
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(AppError::DatabaseQueryError)?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::DuplicateNamedEntity("Division".to_owned()))
    } else {
        Ok(())
    }
}
