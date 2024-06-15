use std::fmt::Display;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use chrono::Datelike;
use chrono::{Duration, Weekday};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaveRequest {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    date_from: NaiveDate,
    date_to: NaiveDate,
    first_day_half: bool,
    last_day_half: bool,
    total_duration: f64,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl Default for LeaveRequest {
    fn default() -> Self {
        LeaveRequest {
            id: None,
            employee_id: 0,
            leave_type_id: 0,
            leave_status: LeaveStatus::default(),
            date_from: Utc::now().date_naive(),
            date_to: Utc::now().date_naive(),
            first_day_half: false,
            last_day_half: false,
            total_duration: 0.0,
            date_requested: Utc::now(),
            date_finalised: None,
            notes: None,
        }
    }
}

impl LeaveRequest {
    pub fn get_date_from(&self) -> NaiveDate {
        self.date_from
    }

    pub fn set_date_from(&mut self, date_from: NaiveDate) {
        self.date_from = date_from;
        self.calculate_requested_days();
    }

    pub fn get_date_to(&self) -> NaiveDate {
        self.date_to
    }

    pub fn set_date_to(&mut self, date_to: NaiveDate) {
        self.date_to = date_to;
        self.calculate_requested_days();
    }

    pub fn get_first_last_day_half(&self) -> (bool, bool) {
        (self.first_day_half, self.last_day_half)
    }

    pub fn set_first_last_day_half(&mut self, hdo: (bool, bool)) {
        (self.first_day_half, self.last_day_half) = hdo;
        self.calculate_requested_days();
    }

    pub fn get_total_duration(&self) -> f64 {
        self.total_duration
    }

    pub fn calculate_requested_days(&mut self) {
        let mut duration_in_days = 0.0;

        let mut current_date = self.date_from;
        while current_date <= self.date_to {
            if current_date.weekday() != Weekday::Sat && current_date.weekday() != Weekday::Sun {
                duration_in_days += 1.0;
            }
            current_date += Duration::days(1);
        }

        // Adjust for half day options
        duration_in_days -=
            HalfDayOptions::from_bools((self.first_day_half, self.last_day_half)).to_f64();

        // Ensure duration_in_days is non-negative
        if duration_in_days < 0.0 {
            duration_in_days = 0.0;
        }

        self.total_duration = duration_in_days;
    }
}
