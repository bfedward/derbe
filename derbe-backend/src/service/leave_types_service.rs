use derbe_shared::dto::request::leave_type::LeaveTypeCreate;

use crate::{
    error::AppError, http::AppContext, model::leave_type::LeaveType,
    repository::leave_types_repository,
};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveType, AppError> {
    leave_types_repository::get_entity_by_id(id, ctx).await
}

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<LeaveType>, AppError> {
    leave_types_repository::get_entities(ctx).await
}

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    leave_types_repository::check_entity_exists(id, ctx).await
}

pub async fn create_entity(dto: LeaveTypeCreate, ctx: &AppContext) -> Result<LeaveType, AppError> {
    pre_create(&dto, ctx).await?;
    leave_types_repository::create_entity(dto, ctx).await
}

pub async fn pre_create(dto: &LeaveTypeCreate, ctx: &AppContext) -> Result<(), AppError> {
    leave_types_repository::check_if_entity_exists_by_name(&dto.name, ctx).await
}
