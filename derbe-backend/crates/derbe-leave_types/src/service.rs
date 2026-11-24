use crate::dto::request::LeaveTypeCreate;
use derbe_core::error::AppError;
use derbe_core::state::AppState;

use crate::{dto::response::LeaveTypeResponse, repository};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LeaveTypeResponse, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<LeaveTypeResponse>, AppError> {
    repository::list(ctx).await
}

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn create(dto: LeaveTypeCreate, ctx: &AppState) -> Result<LeaveTypeResponse, AppError> {
    repository::create(dto, ctx).await
}
