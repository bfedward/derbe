use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    dto::response::employee::EmployeeDetailsResponse,
    error::AppError,
    http::AppContext,
    model::employee::Employee,
    service::{employee_service::EmployeeService, DerbeService},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/employees", get(get_employees))
        .route("/employees/:id", get(get_employee))
        .route("/employees/detailsTable", get(get_employee_details_table))
        .route("/employees", post(create_employee))
        .route("/employees/:id", put(update_employee))
}

async fn get_employees(State(ctx): State<AppContext>) -> Result<Json<Vec<Employee>>, AppError> {
    match EmployeeService::get_entities(&ctx).await {
        Ok(employees) => Ok(Json(employees)),
        Err(e) => Err(e),
    }
}

async fn get_employee_details_table(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<EmployeeDetailsResponse>>, AppError> {
    match EmployeeService::get_employee_details_table(&ctx).await {
        Ok(employees) => Ok(Json(employees)),
        Err(e) => Err(e),
    }
}

async fn get_employee(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<Employee>, AppError> {
    match EmployeeService::get_entity_by_id(id, &ctx).await {
        Ok(employee) => Ok(Json(employee)),
        Err(e) => Err(e),
    }
}

async fn create_employee(
    State(ctx): State<AppContext>,
    Json(leave_request): Json<Employee>,
) -> Result<Response, AppError> {
    match EmployeeService::create_entity(leave_request, &ctx).await {
        Ok(leave_request) => {
            let response = (StatusCode::CREATED, Json(leave_request)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

async fn update_employee(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(employee): Json<Employee>,
) -> Result<Json<Employee>, AppError> {
    match EmployeeService::update_entity(id, employee, &ctx).await {
        Ok(employee) => Ok(Json(employee)),
        Err(e) => Err(e),
    }
}
