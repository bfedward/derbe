use crate::features::divisions::dto::request::DivisionCreate;

use crate::{
    error::AppError, features::divisions::dto::response::DivisionResponse,
    features::divisions::repository, http::AppState,
};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<DivisionResponse, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<DivisionResponse>, AppError> {
    repository::list(ctx).await
}

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn create(dto: DivisionCreate, ctx: &AppState) -> Result<DivisionResponse, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &DivisionCreate, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_name_available(&dto.name, ctx).await
}
