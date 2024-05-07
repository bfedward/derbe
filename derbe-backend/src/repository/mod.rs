use crate::{error::AppError, http::AppContext, model::DerbeModel};

pub mod employee_repository;
pub mod leave_request_respository;
pub mod leave_types_repository;

pub trait DerbeRepository<M: DerbeModel> {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<M, AppError>;

    async fn get_entities(ctx: &AppContext) -> Result<Vec<M>, AppError>;

    async fn create_entity(new_entity: M, ctx: &AppContext) -> Result<M, AppError>;

    async fn update_entity(entity_update: M, ctx: &AppContext) -> Result<M, AppError>;

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError>;
}
