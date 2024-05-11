use crate::{error::AppError, http::AppContext, model::DerbeModel, repository::DerbeRepository};

pub mod division_service;
pub mod employee_service;
pub mod leave_request_service;
pub mod leave_types_service;
pub mod location_service;

pub trait DerbeService<M: DerbeModel, R: DerbeRepository<M>> {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<M, AppError> {
        R::get_entity_by_id(id, ctx).await
    }

    async fn get_entities(ctx: &AppContext) -> Result<Vec<M>, AppError> {
        R::get_entities(ctx).await
    }

    async fn pre_create(_new_entity: &M, _ctx: &AppContext) -> Result<(), AppError> {
        Ok(())
    }

    async fn create_entity(new_entity: M, ctx: &AppContext) -> Result<M, AppError> {
        Self::pre_create(&new_entity, ctx).await?;
        R::create_entity(new_entity, ctx).await
    }

    async fn update_entity(id: i32, entity_update: M, ctx: &AppContext) -> Result<M, AppError> {
        R::update_entity(id, entity_update, ctx).await
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        R::delete_entity(id, ctx).await
    }
}
