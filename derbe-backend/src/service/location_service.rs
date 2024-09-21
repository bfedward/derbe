use derbe_shared::dto::request::location::LocationCreate;

use crate::{
    error::AppError,
    http::AppContext,
    model::location::Location,
    repository::location_repository::{self},
};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Location, AppError> {
    location_repository::get_entity_by_id(id, ctx).await
}

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<Location>, AppError> {
    location_repository::get_entities(ctx).await
}

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    location_repository::check_entity_exists(id, ctx).await
}

pub async fn create_entity(dto: LocationCreate, ctx: &AppContext) -> Result<Location, AppError> {
    pre_create(&dto, ctx).await?;
    location_repository::create_entity(dto, ctx).await
}

pub async fn pre_create(dto: &LocationCreate, ctx: &AppContext) -> Result<(), AppError> {
    location_repository::check_if_entity_exists_by_name(&dto.name, ctx).await
}
