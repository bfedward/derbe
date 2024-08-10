use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "with_sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with_sqlx", sqlx(type_name = "valid_employment_types"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmploymentType {
    #[default]
    Permanent,
    Contract,
}

impl std::fmt::Display for EmploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmploymentType::Permanent => write!(f, "Permanent"),
            EmploymentType::Contract => write!(f, "Contract"),
        }
    }
}

#[cfg_attr(feature = "with_sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with_sqlx", sqlx(type_name = "valid_leave_statuses"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    pub fn from_bools(hdo: (bool, bool)) -> HalfDayOptions {
        match (hdo.0, hdo.1) {
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
