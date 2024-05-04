use axum::{extract::State, routing::get, Json, Router};

use crate::{error::AppError, http::AppContext, model::employee::Employee, service::{employee_service::EmployeeService, DerbeService}};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/employees", get(get_employees))
}

async fn get_employees(State(ctx): State<AppContext>) -> Result<Json<Vec<Employee>>, AppError> {
    match EmployeeService::get_entities(&ctx).await {
        Ok(employees) => Ok(Json(employees)),
        Err(e) => Err(e)
    }
}