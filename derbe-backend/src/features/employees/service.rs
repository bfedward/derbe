use std::str::FromStr;

use uuid::Uuid;

use crate::features::employees::{
    dto::{
        request::{EmployeeCreate, EmployeeUpdate},
        response::EmployeeDetailsResponse,
    }, model::{CreatedEmployee, EmployeeAccess},
};

use crate::{
    error::AppError, features::employees::model::Employee, features::employees::repository,
    http::AppState,
};

use crate::features::{divisions, locations};

pub async fn ensure_exists(unid: Uuid, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(unid, ctx).await
}

pub async fn get_by_id(unid: Uuid, ctx: &AppState) -> Result<Employee, AppError> {
    repository::get_by_id(unid, ctx).await
}

pub async fn get_access_by_id(unid: Uuid, ctx: &AppState) -> Result<EmployeeAccess, AppError> {
    repository::get_access_by_id(unid, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<Employee>, AppError> {
    repository::list(ctx).await
}

pub async fn create(dto: EmployeeCreate, ctx: &AppState) -> Result<CreatedEmployee, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &EmployeeCreate, ctx: &AppState) -> Result<(), AppError> {
    let manager_unid: Option<Uuid> = dto
        .manager_unid
        .as_deref()
        .map(Uuid::from_str)
        .transpose()
        .map_err(|_| AppError::InvalidUuid(dto.manager_unid.clone().unwrap_or_default()))?;

    ensure_related_entities_exist(dto.location_id, dto.division_id, manager_unid, ctx).await?;

    Ok(())
}

pub async fn update(unid: Uuid, dto: EmployeeUpdate, ctx: &AppState) -> Result<Employee, AppError> {
    validate_update(unid, &dto, ctx).await?;
    repository::update(unid, dto, ctx).await
}

pub async fn validate_update(
    unid: Uuid,
    dto: &EmployeeUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    validate_update_rules(unid, dto)?;

    let manager_unid: Option<Uuid> = dto
        .manager_unid
        .as_deref()
        .map(Uuid::from_str)
        .transpose()
        .map_err(|_| AppError::InvalidUuid(dto.manager_unid.clone().unwrap_or_default()))?;

    ensure_related_entities_exist(dto.location_id, dto.division_id, manager_unid, ctx).await?;

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
        get_by_id(manager_id, ctx).await?;
    }
    Ok(())
}

pub fn validate_update_rules(unid: Uuid, dto: &EmployeeUpdate) -> Result<(), AppError> {
    let manager_unid: Option<Uuid> = dto
        .manager_unid
        .as_deref()
        .map(Uuid::from_str)
        .transpose()
        .map_err(|_| AppError::InvalidUuid(dto.manager_unid.clone().unwrap_or_default()))?;

    if let Some(manager_unid) = manager_unid {
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
