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

    async fn pre_create(entity: &M, ctx: &AppContext) -> Result<(), AppError> {
        Self::check_related_entities_exist(entity, ctx).await?;
        Ok(())
    }

    async fn create_entity(entity: M, ctx: &AppContext) -> Result<M, AppError> {
        Self::pre_create(&entity, ctx).await?;
        R::create_entity(entity, ctx).await
    }

    async fn pre_update(_id: i32, entity: &M, ctx: &AppContext) -> Result<(), AppError> {
        Self::check_related_entities_exist(entity, ctx).await?;
        Ok(())
    }

    async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        R::check_entity_exists(id, ctx).await
    }

    async fn check_related_entities_exist(_entity: &M, _ctx: &AppContext) -> Result<(), AppError> {
        Ok(())
    }

    async fn update_entity(id: i32, entity_update: M, ctx: &AppContext) -> Result<M, AppError> {
        Self::pre_update(id, &entity_update, ctx).await?;
        R::update_entity(id, entity_update, ctx).await
    }

    async fn pre_delete(_id: i32, _ctx: &AppContext) -> Result<(), AppError> {
        Ok(())
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        Self::pre_delete(id, ctx).await?;
        R::delete_entity(id, ctx).await
    }
}
