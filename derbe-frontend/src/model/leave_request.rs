use std::fmt::Display;

use chrono::{DateTime, NaiveDate, Utc};
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

impl HalfDayOptions {
    pub fn from_bools(first_day_half: bool, last_day_half: bool) -> HalfDayOptions {
        match (first_day_half, last_day_half) {
            (true, true) => HalfDayOptions::BothHalfDays,
            (true, false) => HalfDayOptions::FirstDayHalf,
            (false, true) => HalfDayOptions::LastDayHalf,
            (false, false) => HalfDayOptions::FullDays,
        }
    }

    pub fn to_bools(&self) -> (bool, bool) {
        match self {
            HalfDayOptions::FullDays => (false, false),
            HalfDayOptions::FirstDayHalf => (true, false),
            HalfDayOptions::LastDayHalf => (false, true),
            HalfDayOptions::BothHalfDays => (true, true),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            HalfDayOptions::FullDays => 0.0,
            HalfDayOptions::FirstDayHalf | HalfDayOptions::LastDayHalf => 0.5,
            HalfDayOptions::BothHalfDays => 1.0,
        }
    }

    pub fn from_string(s: &str) -> HalfDayOptions {
        match s {
            "firstDayHalf" => HalfDayOptions::FirstDayHalf,
            "lastDayHalf" => HalfDayOptions::LastDayHalf,
            "firstAndLastHalfs" => HalfDayOptions::BothHalfDays,
            "allDaysFull" => HalfDayOptions::FullDays,
            _ => panic!("Unknown HalfDayOption: {}", s),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LeaveRequest {
    pub id: Option<i32>,
    pub employee_id: i32,
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
