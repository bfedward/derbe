use std::future::Future;

use crate::{error::AppError, http::AppContext, model::DerbeModel};

pub mod division_repository;
pub mod employee_repository;
pub mod leave_request_respository;
pub mod leave_types_repository;
pub mod location_repository;

pub trait DerbeRepository<M: DerbeModel> {
    fn get_entity_by_id(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send;

    fn get_entities(ctx: &AppContext) -> impl Future<Output = Result<Vec<M>, AppError>> + Send;

    fn create_entity(
        new_entity: M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send;

    fn update_entity(
        id: i32,
        entity_update: M,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<M, AppError>> + Send;

    fn delete_entity(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send;

    fn check_entity_exists(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send;

    fn check_if_entity_is_referenced(
        id: i32,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}
