use std::future::Future;

use crate::{error::AppError, http::AppContext, model::DerbeModel, repository::DerbeRepository};

pub mod division_service;
pub mod employee_service;
pub mod leave_request_service;
pub mod leave_types_service;
pub mod location_service;

pub trait DerbeService<M: DerbeModel + Sync + Send, R: DerbeRepository<M>> {
    fn get_entity_by_id(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send {
        async move { R::get_entity_by_id(id, ctx).await }
    }

    fn get_entities(ctx: &AppContext) -> impl Future<Output = Result<Vec<M>, AppError>> + Send {
        async { R::get_entities(ctx).await }
    }

    fn pre_create(
        entity: &M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send {
        async {
            Self::check_related_entities_exist(entity, ctx).await?;
            Ok(())
        }
    }

    fn create_entity(
        entity: M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send {
        async {
            Self::pre_create(&entity, ctx).await?;
            R::create_entity(entity, ctx).await
        }
    }

    fn pre_update(
        _id: i32,
        entity: &M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send {
        async {
            Self::check_related_entities_exist(entity, ctx).await?;
            Ok(())
        }
    }

    fn check_entity_exists(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send {
        async move { R::check_entity_exists(id, ctx).await }
    }

    fn check_related_entities_exist(
        _entity: &M,
        _ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send;

    fn update_entity(
        id: i32,
        entity_update: M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send {
        async move {
            Self::pre_update(id, &entity_update, ctx).await?;
            R::update_entity(id, entity_update, ctx).await
        }
    }

    fn pre_delete(id: i32, ctx: &AppContext) -> impl Future<Output = Result<(), AppError>> + Send {
        async move {
            R::check_if_entity_is_referenced(id, ctx).await?;
            Ok(())
        }
    }

    fn delete_entity(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send {
        async move {
            Self::pre_delete(id, ctx).await?;
            R::delete_entity(id, ctx).await
        }
    }
}
