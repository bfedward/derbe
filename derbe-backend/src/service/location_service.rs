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
