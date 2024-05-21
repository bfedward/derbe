use crate::{
    error::AppError, http::AppContext, model::division::Division,
    repository::division_repository::DivisionRepository,
};

use super::DerbeService;

pub struct DivisionService {}

impl DerbeService<Division, DivisionRepository> for DivisionService {
    async fn check_related_entities_exist(
        _entity: &Division,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn pre_create_validation(entity: &Division, ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }

    async fn pre_update_validation(
        id: i32,
        entity: &Division,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }
    
    fn pre_create_and_pre_update_validation(entity: &Division) -> Result<(), AppError> {
        todo!()
    }
}
