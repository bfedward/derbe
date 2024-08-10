use crate::{
    error::AppError, http::AppContext, model::division::Division, repository::division_repository,
};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Division, AppError> {
    division_repository::get_entity_by_id(id, ctx).await
}

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<Division>, AppError> {
    division_repository::get_entities(ctx).await
}

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    division_repository::check_entity_exists(id, ctx).await
}
