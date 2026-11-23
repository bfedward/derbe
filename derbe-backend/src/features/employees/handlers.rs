use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    access_control::{authorize, authorize_on},
    error::AppError,
    features::{
        auth::middleware::AuthenticatedUser,
        employees::{
            dto::{
                request::{EmployeeCreate, EmployeeUpdate},
                response::{CreatedEmployeeResponse, EmployeeDetailsResponse, EmployeeResponse},
            },
            permission::{self, EmployeePermission},
            service,
        },
    },
    http::AppState,
};

pub async fn list_employees(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    authorize(&user, EmployeePermission::List)?;
    Ok(Json(service::list(&state).await?))
}

pub async fn get_employee_details_table(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeDetailsResponse>>, AppError> {
    authorize(&user, EmployeePermission::List)?;
    let employees = service::get_employee_details_table(&state).await?;
    Ok(Json(employees))
}

pub async fn get_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(unid): Path<String>,
) -> Result<Json<EmployeeResponse>, AppError> {
    authorize(&user, EmployeePermission::View)?;
    let unid = Uuid::from_str(&unid).map_err(|_| AppError::InvalidUuid(unid))?;
    let employee = service::get_by_id(unid, &state).await?;
    Ok(Json(employee))
}

pub async fn create_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<EmployeeCreate>,
) -> Result<(StatusCode, Json<CreatedEmployeeResponse>), AppError> {
    permission::authorize_create(&user, &input)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;

    let employee = service::create(input, &state).await?;

    Ok((StatusCode::CREATED, Json(employee)))
}

pub async fn update_employee(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(unid): Path<String>,
    Json(input): Json<EmployeeUpdate>,
) -> Result<Json<EmployeeResponse>, AppError> {
    let employee_unid = Uuid::from_str(&unid).map_err(|_| AppError::InvalidUuid(unid))?;
    let employee = service::get_access_by_id(employee_unid, &state).await?;
    authorize_on(&user, &employee, EmployeePermission::Update)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let employee = service::update(employee_unid, input, &state).await?;
    Ok(Json(employee))
}
