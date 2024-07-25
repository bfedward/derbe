use derbe_shared::dto::response::employee::EmployeeDetailsResponse;

use crate::{
    error::AppError, http::AppContext,
    model::employee::Employee, repository::employee_repository::EmployeeRepository,
};

use super::{division_service::DivisionService, location_service::LocationService, DerbeService};

pub struct EmployeeService {}

impl DerbeService<Employee, EmployeeRepository> for EmployeeService {
    async fn pre_update(id: i32, entity: &Employee, ctx: &AppContext) -> Result<(), AppError> {
        Self::pre_update_validation(id, entity, ctx).await?;
        Self::check_related_entities_exist(entity, ctx).await?;
        Ok(())
    }

    async fn check_related_entities_exist(
        entity: &Employee,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        LocationService::check_entity_exists(entity.location_id, ctx).await?;
        DivisionService::check_entity_exists(entity.division_id, ctx).await?;
        if let Some(manager_id) = entity.manager_id {
            Self::get_entity_by_id(manager_id, ctx).await?;
        }
        Ok(())
    }

    async fn pre_create_validation(entity: &Employee, _ctx: &AppContext) -> Result<(), AppError> {
        Self::pre_create_and_pre_update_validation(entity)?;
        Ok(())
    }

    async fn pre_update_validation(
        id: i32,
        entity: &Employee,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Self::pre_create_and_pre_update_validation(entity)?;
        if let Some(manager_id) = entity.manager_id {
            if id == manager_id {
                return Err(AppError::ConstraintError(
                    "An employee cannot be their own manager!".to_owned(),
                ));
            }
        }
        Ok(())
    }

    fn pre_create_and_pre_update_validation(_entity: &Employee) -> Result<(), AppError> {
        Ok(())
    }
}

impl EmployeeService {
    pub async fn get_employee_details_table(
        ctx: &AppContext,
    ) -> Result<Vec<EmployeeDetailsResponse>, AppError> {
        EmployeeRepository::get_employee_details_table(ctx).await
    }
}
