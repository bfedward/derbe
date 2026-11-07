use crate::features::locations::dto::request::LocationCreate;

use crate::{
    error::AppError,
    features::locations::model::Location,
    features::locations::repository::{self},
    http::AppState,
};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<Location, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<Location>, AppError> {
    repository::list(ctx).await
}

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn create(dto: LocationCreate, ctx: &AppState) -> Result<Location, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &LocationCreate, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_name_available(&dto.name, ctx).await
}
