use crate::{
    error::AppError, http::AppContext, model::leave_type::LeaveType, repository::leave_types_repository::LeaveTypesRepository
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

    async fn pre_create_validation(
        entity: &LeaveType,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }

    async fn pre_update_validation(
        id: i32,
        entity: &LeaveType,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        todo!()
    }
    
    fn pre_create_and_pre_update_validation(entity: &LeaveType) -> Result<(), AppError> {
        todo!()
    }
}
