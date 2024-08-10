use derbe_shared::dto::request::employee::{EmployeeCreate, EmployeeUpdate};
use derbe_shared::dto::response::employee::{EmployeeDetailsResponse, EmployeeResponse};
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;

pub async fn get_employees() -> Result<Vec<EmployeeResponse>, ApiError> {
    let req = Request::get(&format!("{}/employees", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<EmployeeResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_employee_details_table() -> Result<Vec<EmployeeDetailsResponse>, ApiError> {
    let req = Request::get(&format!("{}/employees/detailsTable", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<EmployeeDetailsResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_employee(id: &i32) -> Result<EmployeeResponse, ApiError> {
    let req = Request::get(&format!("{}/employees/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<EmployeeResponse>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn create_employee(employee: &EmployeeCreate) -> Result<EmployeeResponse, ApiError> {
    let request = Request::post(&format!("{}/employees", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(employee).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<EmployeeResponse>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn update_employee(
    id: &i32,
    employee: &EmployeeUpdate,
) -> Result<EmployeeResponse, ApiError> {
    let request = Request::put(&format!("{}/employees/{}", BASE_URL, id))
        .header("content-type", "application/json")
        .body(serde_json::to_string(employee).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<EmployeeResponse>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}
