use crate::features::locations::dto::request::LocationCreate;

use crate::{
    error::AppError,
    features::locations::dto::response::LocationResponse,
    features::locations::repository::{self},
    http::AppState,
};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LocationResponse, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<LocationResponse>, AppError> {
    repository::list(ctx).await
}

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn create(dto: LocationCreate, ctx: &AppState) -> Result<LocationResponse, AppError> {
    repository::create(dto, ctx).await
}
