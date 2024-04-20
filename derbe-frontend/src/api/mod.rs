pub mod api_errors;

use reqwasm::http::Request;

use self::api_errors::ApiError;
use crate::model::leave_type::LeaveType;

const BASE_URL: &str = include_str!("api_base_uri.txt");


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

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}