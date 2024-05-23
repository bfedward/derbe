use crate::{
    error::AppError, http::AppContext, model::location::Location,
    repository::location_repository::LocationRepository,
};

use super::DerbeService;

pub struct LocationService {}

impl DerbeService<Location, LocationRepository> for LocationService {
    async fn check_related_entities_exist(
        _entity: &Location,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn pre_create_validation(_entity: &Location, _ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }

    async fn pre_update_validation(
        _id: i32,
        _entity: &Location,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }

    fn pre_create_and_pre_update_validation(_entity: &Location) -> Result<(), AppError> {
        todo!()
    }
}
