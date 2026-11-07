use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::AppError,
    features::employees::dto::{
        request::{EmployeeCreate, EmployeeUpdate},
        response::{EmployeeDetailsResponse, EmployeeResponse},
    },
    features::employees::service,
    http::AppState,
};

pub async fn list_employees(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    let employees = service::list(&state).await?;
    Ok(Json(
        employees.into_iter().map(EmployeeResponse::from).collect(),
    ))
}

pub async fn get_employee_details_table(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeDetailsResponse>>, AppError> {
    let employees = service::get_employee_details_table(&state).await?;
    Ok(Json(employees))
}

pub async fn get_employee(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<EmployeeResponse>, AppError> {
    let employee = service::get_by_id(id, &state).await?;
    Ok(Json(employee.into()))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(input): Json<EmployeeCreate>,
) -> Result<(StatusCode, Json<EmployeeResponse>), AppError> {
    let employee = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(employee.into())))
}

pub async fn update_employee(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<EmployeeUpdate>,
) -> Result<Json<EmployeeResponse>, AppError> {
    let employee = service::update(id, input, &state).await?;
    Ok(Json(employee.into()))
}
