use crate::{
    error::AppError, http::AppContext, model::employee::Employee,
    repository::employee_repository::EmployeeRepository,
};

use super::{division_service::DivisionService, location_service::LocationService, DerbeService};

pub struct EmployeeService {}

impl DerbeService<Employee, EmployeeRepository> for EmployeeService {
    async fn pre_create(new_entity: &Employee, ctx: &AppContext) -> Result<(), AppError> {
        // need to check: location, division, manager
        LocationService::get_entity_by_id(new_entity.location_id, ctx).await?;
        DivisionService::get_entity_by_id(new_entity.division_id, ctx).await?;
        if let Some(manager_id) = new_entity.manager_id {
            Self::get_entity_by_id(manager_id, ctx).await?;
        }
        Ok(())
    }
}
