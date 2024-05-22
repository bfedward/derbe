use serde::{Deserialize, Serialize};

use crate::model::leave_request::LeaveStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyLeaveRequest {
    pub employee_id: i32,
    pub leave_type_ids: Vec<i32>,
    pub leave_statuses: Vec<LeaveStatus>,
    pub date_from: String,
    pub date_to: String,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct MyLeaveResponse {
    pub leave_request_id: i32,
    pub leave_type: String,
    pub leave_status: LeaveStatus,
    pub date_from: String,
    pub date_to: String,
    pub total_duration: f64,
}
