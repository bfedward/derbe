use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum LeaveStatus {
    #[default]
    Pending,
    MoreInfoReq,
    Cancelled,
    Rejected,
    Accepted,
}

impl Display for LeaveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaveStatus::Pending => write!(f, "Pending"),
            LeaveStatus::MoreInfoReq => write!(f, "MoreInfoReq"),
            LeaveStatus::Cancelled => write!(f, "Cancelled"),
            LeaveStatus::Rejected => write!(f, "Rejected"),
            LeaveStatus::Accepted => write!(f, "Accepted"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum HalfDayOptions {
    #[default]
    FullDays,
    FirstDayHalf,
    LastDayHalf,
    BothHalfDays,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LeaveRequest {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub first_day_half: bool,
    pub last_day_half: bool,
    pub total_duration: f64,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
