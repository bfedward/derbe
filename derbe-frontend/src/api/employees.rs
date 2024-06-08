use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::dto::employee::EmployeeDetailsResponse;
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
