use derbe_shared::dto::{
    request::employee::{EmployeeCreate, EmployeeUpdate},
    response::employee::EmployeeDetailsResponse,
};

use crate::{
    error::AppError, http::AppContext, model::employee::Employee, repository::employee_repository,
};

use super::{division_service, location_service};

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    employee_repository::check_entity_exists(id, ctx).await
}

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Employee, AppError> {
    employee_repository::get_entity_by_id(id, ctx).await
}

pub async fn get_entities(ctx: &AppContext) -> Result<Vec<Employee>, AppError> {
    employee_repository::get_entities(ctx).await
}

pub async fn create_entity(dto: EmployeeCreate, ctx: &AppContext) -> Result<Employee, AppError> {
    pre_create(&dto, ctx).await?;
    employee_repository::create_entity(dto, ctx).await
}

pub async fn pre_create(dto: &EmployeeCreate, ctx: &AppContext) -> Result<(), AppError> {
    check_related_entities_exist(dto.location_id, dto.division_id, dto.manager_id, ctx).await?;

    Ok(())
}

pub async fn update_entity(
    id: i32,
    dto: EmployeeUpdate,
    ctx: &AppContext,
) -> Result<Employee, AppError> {
    pre_update(id, &dto, ctx).await?;
    employee_repository::update_entity(id, dto, ctx).await
}

pub async fn pre_update(id: i32, dto: &EmployeeUpdate, ctx: &AppContext) -> Result<(), AppError> {
    pre_update_validation(id, dto, ctx).await?;
    check_related_entities_exist(dto.location_id, dto.division_id, dto.manager_id, ctx).await?;
    Ok(())
}

pub async fn check_related_entities_exist(
    location_id: i32,
    division_id: i32,
    manager_id: Option<i32>,
    ctx: &AppContext,
) -> Result<(), AppError> {
    location_service::check_entity_exists(location_id, ctx).await?;
    division_service::check_entity_exists(division_id, ctx).await?;
    if let Some(manager_id) = manager_id {
        get_entity_by_id(manager_id, ctx).await?;
    }
    Ok(())
}

pub async fn pre_update_validation(
    id: i32,
    dto: &EmployeeUpdate,
    _ctx: &AppContext,
) -> Result<(), AppError> {
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
    ctx: &AppContext,
) -> Result<Vec<EmployeeDetailsResponse>, AppError> {
    employee_repository::get_employee_details_table(ctx).await
}
