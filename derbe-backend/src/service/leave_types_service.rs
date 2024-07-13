use crate::{
    error::AppError, http::AppContext, model::leave_type::LeaveType,
    repository::leave_types_repository::LeaveTypesRepository,
};

use super::DerbeService;

pub struct LeaveTypesService {}

impl DerbeService<LeaveType, LeaveTypesRepository> for LeaveTypesService {
    async fn check_related_entities_exist(
        _entity: &LeaveType,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn pre_create_validation(_entity: &LeaveType, _ctx: &AppContext) -> Result<(), AppError> {
        Ok(())
    }

    async fn pre_update_validation(
        _id: i32,
        _entity: &LeaveType,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Ok(())
    }

    fn pre_create_and_pre_update_validation(_entity: &LeaveType) -> Result<(), AppError> {
        Ok(())
    }
}
