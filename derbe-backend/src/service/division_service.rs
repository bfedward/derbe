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
}
