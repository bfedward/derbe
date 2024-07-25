use derbe_shared::dto::response::employee::EmployeeDetailsResponse;
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::model::employee::Employee;

pub async fn get_employees() -> Result<Vec<Employee>, ApiError> {
    let req = Request::get(&format!("{}/employees", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<Employee>>().await.unwrap())
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

pub async fn get_employee(id: &i32) -> Result<Employee, ApiError> {
    let req = Request::get(&format!("{}/employees/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Employee>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn create_employee(employee: &Employee) -> Result<Employee, ApiError> {
    let request = Request::post(&format!("{}/employees", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(employee).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<Employee>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn update_employee(employee: &Employee) -> Result<Employee, ApiError> {
    let request = Request::put(&format!("{}/employees/{}", BASE_URL, employee.id.unwrap()))
        .header("content-type", "application/json")
        .body(serde_json::to_string(employee).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<Employee>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}
