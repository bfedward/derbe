use chrono::{Datelike, NaiveDate, Utc};
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::dto::leave_request::{LeaveRequestStatusUpdate, MyLeaveRequest, MyLeaveResponse};
use crate::model::leave_request::{HalfDayOptions, LeaveRequest, LeaveStatus};

pub async fn get_leave_request(id: &i32) -> Result<LeaveRequest, ApiError> {
    let request = Request::get(&format!("{}/leaveRequests/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<LeaveRequest>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn get_my_leave() -> Result<Vec<MyLeaveResponse>, ApiError> {
    let current_year = Utc::now().year();
    let start_jan: NaiveDate = NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap();
    let end_dec: NaiveDate = NaiveDate::from_ymd_opt(current_year + 1, 12, 31).unwrap();

    let my_leave_request = MyLeaveRequest {
        employee_id: 3,
        leave_type_ids: vec![1, 5],
        leave_statuses: vec![LeaveStatus::Pending],
        date_from: start_jan,
        date_to: end_dec,
    };

    let req = Request::post(&format!("{}/myLeave", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&my_leave_request).unwrap())
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<MyLeaveResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn create_leave_request(
    employee_id: i32,
    leave_type_id: i32,
    date_from: NaiveDate,
    date_to: NaiveDate,
    half_day_opt: HalfDayOptions,
    total_duration: f64,
    notes: Option<String>,
) -> Result<LeaveRequest, ApiError> {
    let (first_day_half, last_day_half) = HalfDayOptions::to_bools(&half_day_opt);

    let new_leave_request = LeaveRequest {
        id: None,
        employee_id,
        leave_type_id,
        leave_status: LeaveStatus::Pending,
        date_from,
        date_to,
        first_day_half,
        last_day_half,
        total_duration,
        date_requested: Utc::now(),
        date_finalised: None,
        notes,
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

pub async fn update_leave_request(leave_request: &LeaveRequest) -> Result<LeaveRequest, ApiError> {
    let request = Request::put(&format!(
        "{}/leaveRequests/{}",
        BASE_URL,
        leave_request.id.unwrap()
    ))
    .header("content-type", "application/json")
    .body(serde_json::to_string(leave_request).unwrap())
    .send()
    .await
    .unwrap();

    if request.ok() {
        Ok(request.json::<LeaveRequest>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn update_leave_request_status(
    id: &i32,
    status_update: LeaveRequestStatusUpdate,
) -> Result<LeaveRequest, ApiError> {
    let request = Request::put(&format!("{}/leaveRequests/{}/status", BASE_URL, id))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&status_update).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<LeaveRequest>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}
