use crate::{
    error::AppError, http::AppContext, model::location::Location, repository::location_repository::LocationRepository
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

    async fn pre_create_validation(
        entity: &Location,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }

    async fn pre_update_validation(
        id: i32,
        entity: &Location,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }
    
    fn pre_create_and_pre_update_validation(entity: &Location) -> Result<(), AppError> {
        todo!()
    }
}
