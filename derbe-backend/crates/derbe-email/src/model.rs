use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use derbe_core::model::LeaveStatus;

/// Stored in `email_outbox.template`, e.g. "leave_request_submitted".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EmailTemplate {
    LeaveRequestSubmitted,
    LeaveRequestDecision,
}

/// Sent to the manager when an employee submits a request.
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRequestSubmittedPayload {
    pub request_id: i32,
    pub employee_name: String,
    pub leave_type: String,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub total_duration: f64,
    pub notes: Option<String>,
}

/// Sent to the employee when a request is accepted or rejected.
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRequestDecisionPayload {
    pub request_id: i32,
    pub leave_type: String,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub total_duration: f64,
    pub decision: LeaveStatus,
    pub decided_by: String,
}
