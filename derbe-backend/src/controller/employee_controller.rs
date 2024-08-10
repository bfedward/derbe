use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use derbe_shared::dto::{
    request::employee::{EmployeeCreate, EmployeeUpdate},
    response::employee::{EmployeeDetailsResponse, EmployeeResponse},
};

use crate::{error::AppError, http::AppContext, service::employee_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/employees", get(get_employees))
        .route("/employees/:id", get(get_employee))
        .route("/employees/detailsTable", get(get_employee_details_table))
        .route("/employees", post(create_employee))
        .route("/employees/:id", put(update_employee))
}

async fn get_employees(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    match employee_service::get_entities(&ctx).await {
        Ok(employees) => {
            let resps = employees.into_iter().map(EmployeeResponse::from).collect();
            Ok(Json(resps))
        }
        Err(e) => Err(e),
    }
}

async fn get_employee_details_table(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<EmployeeDetailsResponse>>, AppError> {
    match employee_service::get_employee_details_table(&ctx).await {
        Ok(employees) => Ok(Json(employees)),
        Err(e) => Err(e),
    }
}

async fn get_employee(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<EmployeeResponse>, AppError> {
    match employee_service::get_entity_by_id(id, &ctx).await {
        Ok(employee) => {
            let resp = employee.into();
            Ok(Json(resp))
        }
        Err(e) => Err(e),
    }
}

async fn create_employee(
    State(ctx): State<AppContext>,
    Json(employee): Json<EmployeeCreate>,
) -> Result<Response, AppError> {
    match employee_service::create_entity(employee, &ctx).await {
        Ok(employee) => {
            let employee: EmployeeResponse = employee.into();
            let response = (StatusCode::CREATED, Json(employee)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

async fn update_employee(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(employee): Json<EmployeeUpdate>,
) -> Result<Json<EmployeeResponse>, AppError> {
    match employee_service::update_entity(id, employee, &ctx).await {
        Ok(employee) => Ok(Json(employee.into())),
        Err(e) => Err(e),
    }
}
