use crate::features::leave_types::dto::request::LeaveTypeCreate;

use crate::{
    error::AppError, features::leave_types::dto::response::LeaveTypeResponse,
    features::leave_types::repository, http::AppState,
};

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
