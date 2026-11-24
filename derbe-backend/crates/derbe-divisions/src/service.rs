use crate::dto::request::DivisionCreate;
use derbe_core::error::AppError;
use derbe_core::state::AppState;

use crate::{dto::response::DivisionResponse, repository};

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
    repository::create(dto, ctx).await
}
