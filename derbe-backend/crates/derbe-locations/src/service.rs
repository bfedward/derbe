use crate::dto::request::LocationCreate;
use derbe_core::error::AppError;
use derbe_core::state::AppState;

use crate::{
    dto::response::LocationResponse,
    repository::{self},
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
