use chrono::{DateTime, Datelike, TimeZone, Utc};
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::dto::leave_request::{MyLeaveRequest, MyLeaveResponse};
use crate::model::leave_request::{HalfDayOptions, LeaveRequest, LeaveStatus};

pub async fn get_my_leave() -> Result<Vec<MyLeaveResponse>, ApiError> {
    let current_year = Utc::now().year();
    let start_jan: DateTime<Utc> = Utc.with_ymd_and_hms(current_year, 1, 1, 0, 0, 0).unwrap();
    let end_dec: DateTime<Utc> = Utc
        .with_ymd_and_hms(current_year + 1, 1, 1, 0, 0, 0)
        .unwrap();

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
    employee: i32,
    leave_type: i32,
    date_from: DateTime<Utc>,
    date_to: DateTime<Utc>,
    half_day_opt: HalfDayOptions,
    total_duration: f64,
    notes: Option<String>,
) -> Result<LeaveRequest, ApiError> {
    let (first_day_half, last_day_half) = match half_day_opt {
        HalfDayOptions::FullDays => (false, false),
        HalfDayOptions::FirstDayHalf => (true, false),
        HalfDayOptions::LastDayHalf => (false, true),
        HalfDayOptions::BothHalfDays => (true, true),
    };

    let new_leave_request = LeaveRequest {
        id: None,
        employee_id: employee,
        leave_type_id: leave_type,
        leave_status: LeaveStatus::Pending,
        date_from,
        date_to,
        first_day_half,
        last_day_half,
        total_duration,
        date_requested: Utc::now().to_rfc3339(),
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
