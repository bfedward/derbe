use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::enums::LeaveStatus;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveRequestResponse {
    pub id: i32,
    pub employee_unid: String,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub first_day_half: bool,
    pub last_day_half: bool,
    pub total_duration: f64,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AllLeaveResponse {
    pub employee_name: String,
    pub leaves: Vec<AllLeaveInnerResponse>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AllLeaveInnerResponse {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub first_day_half: bool,
    pub last_day_half: bool,
    pub colour: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct MyLeaveResponse {
    pub leave_request_id: i32,
    pub leave_type: String,
    pub leave_status: LeaveStatus,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub total_duration: f64,
}
