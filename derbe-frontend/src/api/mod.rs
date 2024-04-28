pub mod api_errors;

use reqwasm::http::Request;
use time::OffsetDateTime;

use self::api_errors::ApiError;
use crate::model::{leave_request::{HalfDayOptions, LeaveRequest, LeaveStatus}, leave_type::LeaveType};

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

pub async fn create_leave_request(
    leave_type: i32,
    date_from: OffsetDateTime,
    date_to: OffsetDateTime,
    half_day_opt: HalfDayOptions,
    notes: Option<String>
) -> Result<LeaveRequest, ApiError> {
    let (first_day_half, last_day_half) = match half_day_opt {
        HalfDayOptions::FullDays => (false, false),
        HalfDayOptions::FirstDayHalf => (true, false),
        HalfDayOptions::LastDayHalf => (false, true),
        HalfDayOptions::BothHalfDays => (true, true)
    };

    let new_leave_request = LeaveRequest {
        id: None,
        employee_id: 3,
        leave_type_id: leave_type,
        leave_status: LeaveStatus::PENDING,
        date_from,
        date_to,
        first_day_half,
        last_day_half,
        date_requested: OffsetDateTime::now_utc(),
        date_finalised: None,
        notes
    };

    let request = Request::post(&format!("{}/leaveRequests", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&new_leave_request).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<LeaveRequest>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}