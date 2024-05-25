use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::model::leave_type::LeaveType;

pub async fn get_leave_types() -> Result<Vec<LeaveType>, ApiError> {
    let req = Request::get(&format!("{}/leaveTypes", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<LeaveType>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_leave_type(id: &i32) -> Result<LeaveType, ApiError> {
    let req = Request::get(&format!("{}/leaveTypes/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<LeaveType>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
