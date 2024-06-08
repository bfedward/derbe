use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    dto::response::employee::EmployeeDetailsResponse, error::AppError, http::AppContext, model::employee::Employee, service::{employee_service::EmployeeService, DerbeService}
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/employees", get(get_employees))
        .route("/employees/:id", get(get_employee))
        .route("/employees/detailsTable", get(get_employee_details_table))
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
