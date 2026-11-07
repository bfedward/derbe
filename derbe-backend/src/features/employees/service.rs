use crate::features::employees::dto::{
    request::{EmployeeCreate, EmployeeUpdate},
    response::EmployeeDetailsResponse,
};

use crate::{
    error::AppError, features::employees::model::Employee, features::employees::repository,
    http::AppState,
};

use crate::features::{divisions, locations};

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<Employee, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn list(ctx: &AppState) -> Result<Vec<Employee>, AppError> {
    repository::list(ctx).await
}

pub async fn create(dto: EmployeeCreate, ctx: &AppState) -> Result<Employee, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &EmployeeCreate, ctx: &AppState) -> Result<(), AppError> {
    ensure_related_entities_exist(dto.location_id, dto.division_id, dto.manager_id, ctx).await?;

    Ok(())
}

pub async fn update(id: i32, dto: EmployeeUpdate, ctx: &AppState) -> Result<Employee, AppError> {
    validate_update(id, &dto, ctx).await?;
    repository::update(id, dto, ctx).await
}

pub async fn validate_update(
    id: i32,
    dto: &EmployeeUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    validate_update_rules(id, dto)?;
    ensure_related_entities_exist(dto.location_id, dto.division_id, dto.manager_id, ctx).await?;
    Ok(())
}

pub async fn ensure_related_entities_exist(
    location_id: i32,
    division_id: i32,
    manager_id: Option<i32>,
    ctx: &AppState,
) -> Result<(), AppError> {
    locations::service::ensure_exists(location_id, ctx).await?;
    divisions::service::ensure_exists(division_id, ctx).await?;
    if let Some(manager_id) = manager_id {
        get_by_id(manager_id, ctx).await?;
    }
    Ok(())
}

pub fn validate_update_rules(id: i32, dto: &EmployeeUpdate) -> Result<(), AppError> {
    if let Some(manager_id) = dto.manager_id {
        if id == manager_id {
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
