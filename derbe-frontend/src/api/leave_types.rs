use derbe_shared::dto::request::leave_type::LeaveTypeCreate;
use derbe_shared::dto::response::leave_type::LeaveTypeResponse;
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;

pub async fn get_leave_types() -> Result<Vec<LeaveTypeResponse>, ApiError> {
    let req = Request::get(&format!("{}/leaveTypes", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<LeaveTypeResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_leave_type(id: &i32) -> Result<LeaveTypeResponse, ApiError> {
    let req = Request::get(&format!("{}/leaveTypes/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<LeaveTypeResponse>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn create_leave_type(
    leave_type: &LeaveTypeCreate,
) -> Result<LeaveTypeResponse, ApiError> {
    let request = Request::post(&format!("{}/leaveTypes", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&leave_type).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<LeaveTypeResponse>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}
