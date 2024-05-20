use crate::{
    http::AppContext, model::leave_type::LeaveType,
    repository::leave_types_repository::LeaveTypesRepository,
};

use super::DerbeService;

pub struct LeaveTypesService {}

impl DerbeService<LeaveType, LeaveTypesRepository> for LeaveTypesService {
    async fn check_related_entities_exist(
        _entity: &LeaveType,
        _ctx: &AppContext,
    ) -> Result<(), crate::error::AppError> {
        Ok(())
    }
}
