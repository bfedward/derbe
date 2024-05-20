use crate::{
    http::AppContext, model::location::Location,
    repository::location_repository::LocationRepository,
};

use super::DerbeService;

pub struct LocationService {}

impl DerbeService<Location, LocationRepository> for LocationService {
    async fn check_related_entities_exist(
        _entity: &Location,
        _ctx: &AppContext,
    ) -> Result<(), crate::error::AppError> {
        Ok(())
    }
}
