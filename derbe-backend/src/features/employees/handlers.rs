use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    error::AppError,
    features::employees::{
        dto::{
            request::{EmployeeCreate, EmployeeUpdate},
            response::{CreatedEmployeeResponse, EmployeeDetailsResponse, EmployeeResponse},
        },
        service,
    },
    http::AppState,
    permissions::{AccessController, Permission},
};

pub async fn list_employees(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewEmployees)?;
    let employees = service::list(&state).await?;
    Ok(Json(
        employees.into_iter().map(EmployeeResponse::from).collect(),
    ))
}

pub async fn get_employee_details_table(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeDetailsResponse>>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewEmployees)?;
    let employees = service::get_employee_details_table(&state).await?;
    Ok(Json(employees))
}

pub async fn get_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(unid): Path<String>,
) -> Result<Json<EmployeeResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewEmployees)?;
    let unid = Uuid::from_str(&unid).map_err(|_| AppError::InvalidUuid(unid))?;
    let employee = service::get_by_id(unid, &state).await?;
    Ok(Json(employee.into()))
}

pub async fn create_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<EmployeeCreate>,
) -> Result<(StatusCode, Json<CreatedEmployeeResponse>), AppError> {
    AccessController::authorize(&user.role, Permission::CreateEmployee)?;

    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;

    let employee = service::create(input, &state).await?;

    Ok((StatusCode::CREATED, Json(employee.into())))
}

pub async fn update_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(unid): Path<String>,
    Json(input): Json<EmployeeUpdate>,
) -> Result<Json<EmployeeResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::UpdateEmployee)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let unid = Uuid::from_str(&unid).map_err(|_| AppError::InvalidUuid(unid))?;
    let employee = service::update(unid, input, &state).await?;
    Ok(Json(employee.into()))
}
