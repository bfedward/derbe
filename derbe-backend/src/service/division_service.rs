use crate::{
    error::AppError, http::AppContext, model::division::Division,
    repository::division_repository::DivisionRepository,
};

use super::DerbeService;

pub struct DivisionService {}

impl DerbeService<Division, DivisionRepository> for DivisionService {
    async fn pre_delete(_id: i32, _ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }
}
