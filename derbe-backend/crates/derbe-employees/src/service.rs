use derbe_core::state::AppState;
use derbe_core::{
    error::AppError,
    model::{EmployeeAccess, EmployeeRole},
};
use uuid::Uuid;

use crate::dto::{
    request::{EmployeeCreate, EmployeeUpdate},
    response::{CreatedEmployeeResponse, EmployeeDetailsResponse, EmployeeResponse},
};

use crate::repository;

use derbe_divisions as divisions;
use derbe_locations as locations;

pub async fn ensure_exists(unid: Uuid, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(unid, ctx).await
}

pub async fn get_by_id(unid: Uuid, ctx: &AppState) -> Result<EmployeeResponse, AppError> {
    repository::get_by_id(unid, ctx).await
}

pub async fn get_access_by_id(unid: Uuid, ctx: &AppState) -> Result<EmployeeAccess, AppError> {
    repository::get_access_by_id(unid, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<EmployeeResponse>, AppError> {
    repository::list(ctx).await
}

pub async fn create(
    dto: EmployeeCreate,
    ctx: &AppState,
) -> Result<CreatedEmployeeResponse, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &EmployeeCreate, ctx: &AppState) -> Result<(), AppError> {
    ensure_related_entities_exist(dto.location_id, dto.division_id, dto.manager_unid, ctx).await?;

    Ok(())
}

pub async fn update(
    unid: Uuid,
    dto: EmployeeUpdate,
    ctx: &AppState,
) -> Result<EmployeeResponse, AppError> {
    validate_update(unid, &dto, ctx).await?;
    repository::update(unid, dto, ctx).await
}

pub async fn validate_update(
    unid: Uuid,
    dto: &EmployeeUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    validate_update_rules(unid, dto)?;

    ensure_related_entities_exist(dto.location_id, dto.division_id, dto.manager_unid, ctx).await?;

    Ok(())
}

pub async fn ensure_related_entities_exist(
    location_id: i32,
    division_id: i32,
    manager_id: Option<Uuid>,
    ctx: &AppState,
) -> Result<(), AppError> {
    locations::service::ensure_exists(location_id, ctx).await?;
    divisions::service::ensure_exists(division_id, ctx).await?;
    if let Some(manager_id) = manager_id {
        let manager = get_by_id(manager_id, ctx).await?;
        if !matches!(manager.role, EmployeeRole::Admin | EmployeeRole::Manager) {
            return Err(AppError::ValidationFailed(
                "An employee's manager must be an admin or a manager".to_owned(),
            ));
        }
    }
    Ok(())
}

pub fn validate_update_rules(unid: Uuid, dto: &EmployeeUpdate) -> Result<(), AppError> {
    if let Some(manager_unid) = dto.manager_unid {
        if unid == manager_unid {
            return Err(AppError::ConstraintError(
                "An employee cannot be their own manager!".to_owned(),
            ));
        }
    }
    Ok(())
}

pub async fn get_employee_details_table(
    ctx: &AppState,
) -> Result<Vec<EmployeeDetailsResponse>, AppError> {
    repository::get_employee_details_table(ctx).await
}
