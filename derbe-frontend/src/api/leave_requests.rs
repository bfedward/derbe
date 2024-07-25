use chrono::{Datelike, NaiveDate, Utc};
use derbe_shared::dto::request::leave_request::AllLeaveRequest;
use derbe_shared::dto::response::leave_request::AllLeaveResponse;
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::dto::leave_request::{
    LeaveRequestStatusUpdate, MyLeaveRequest, MyLeaveResponse
};
use crate::model::leave_request::{LeaveRequest, LeaveStatus};
use crate::utils::date_utils::get_first_and_last_date;

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
    let end_dec: NaiveDate = NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap();

    let my_leave_request = MyLeaveRequest {
        employee_id: 1,
        leave_type_ids: vec![1, 5],
        leave_statuses: vec![LeaveStatus::Pending],
        date_from: start_jan,
        date_to: end_dec,
    };

    let res = Request::post(&format!("{}/myLeave", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&my_leave_request).unwrap())
        .send()
        .await
        .unwrap();

    if res.ok() {
        Ok(res.json::<Vec<MyLeaveResponse>>().await.unwrap())
    } else {
        Err(handle_errors(res.status()))
    }
}

pub async fn get_all_leave() -> Result<Vec<AllLeaveResponse>, ApiError> {
    let now = Utc::now().naive_utc(); // convert to NaiveDateTime
    let (date_from, date_to) = get_first_and_last_date(now.year(), now.month());

    let req = AllLeaveRequest { date_from, date_to };

    let res = Request::post(&format!("{}/allLeave", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await
        .unwrap();

    if res.ok() {
        Ok(res.json::<Vec<AllLeaveResponse>>().await.unwrap())
    } else {
        Err(handle_errors(res.status()))
    }
}

pub async fn create_leave_request(leave_request: &LeaveRequest) -> Result<LeaveRequest, ApiError> {
    let request = Request::post(&format!("{}/leaveRequests", BASE_URL))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&leave_request).unwrap())
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
