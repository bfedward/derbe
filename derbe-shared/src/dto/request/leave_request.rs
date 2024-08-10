use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

use crate::{
    deserialization::deserialize_non_negative_f64,
    dto::response::leave_request::LeaveRequestResponse,
    enums::{HalfDayOptions, LeaveStatus},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaveRequestCreate {
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub values: LeaveRequestValues,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl Default for LeaveRequestCreate {
    fn default() -> Self {
        LeaveRequestCreate {
            employee_id: 0,
            leave_type_id: 0,
            leave_status: LeaveStatus::default(),
            values: LeaveRequestValues::default(),
            date_requested: Utc::now(),
            date_finalised: None,
            notes: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaveRequestValues {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub first_day_half: bool,
    pub last_day_half: bool,
    #[serde(deserialize_with = "deserialize_non_negative_f64")]
    pub total_duration: f64,
}

impl Default for LeaveRequestValues {
    fn default() -> Self {
        LeaveRequestValues {
            date_from: Utc::now().date_naive(),
            date_to: Utc::now().date_naive(),
            first_day_half: false,
            last_day_half: false,
            total_duration: 0.0,
        }
    }
}

impl LeaveRequestValues {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequestUpdate {
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub values: LeaveRequestValues,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl From<LeaveRequestResponse> for LeaveRequestUpdate {
    fn from(value: LeaveRequestResponse) -> Self {
        LeaveRequestUpdate {
            employee_id: value.employee_id,
            leave_type_id: value.leave_type_id,
            leave_status: value.leave_status,
            values: LeaveRequestValues {
                date_from: value.date_from,
                date_to: value.date_to,
                first_day_half: value.first_day_half,
                last_day_half: value.last_day_half,
                total_duration: value.total_duration,
            },
            date_requested: value.date_requested,
            date_finalised: value.date_finalised,
            notes: value.notes,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllLeaveRequest {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyLeaveRequest {
    pub employee_id: i32,
    pub leave_type_ids: Vec<i32>,
    pub leave_statuses: Vec<LeaveStatus>,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRequestStatusUpdate {
    pub leave_status: LeaveStatus,
    pub notes: Option<String>,
}
