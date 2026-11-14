use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    enums::{HalfDayOptions, LeaveStatus},
    error::AppError,
};

use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone, TS)]
#[serde(try_from = "LeaveRequestCreateRaw")]
#[ts(export)]
pub struct LeaveRequestCreate {
    #[ts(type = "string")]
    pub employee_unid: Uuid,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    #[validate(nested)]
    pub values: LeaveRequestValues,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
struct LeaveRequestCreateRaw {
    employee_unid: String,
    leave_type_id: i32,
    leave_status: LeaveStatus,
    values: LeaveRequestValues,
    date_requested: DateTime<Utc>,
    date_finalised: Option<DateTime<Utc>>,
    notes: Option<String>,
}

impl TryFrom<LeaveRequestCreateRaw> for LeaveRequestCreate {
    type Error = AppError;

    fn try_from(raw: LeaveRequestCreateRaw) -> Result<Self, Self::Error> {
        let employee_unid = Uuid::parse_str(&raw.employee_unid)
            .map_err(|_| AppError::InvalidUuid(raw.employee_unid))?;

        Ok(Self {
            employee_unid,
            leave_type_id: raw.leave_type_id,
            leave_status: raw.leave_status,
            values: raw.values,
            date_requested: raw.date_requested,
            date_finalised: raw.date_finalised,
            notes: raw.notes,
        })
    }
}

#[derive(Debug, Validate, Deserialize, Clone, TS)]
#[ts(export)]
pub struct LeaveRequestValues {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub first_day_half: bool,
    pub last_day_half: bool,
    #[validate(range(min = 1.0))]
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

#[derive(Validate, Debug, Clone, Deserialize, TS)]
#[serde(try_from = "LeaveRequestUpdateRaw")]
#[ts(export)]
pub struct LeaveRequestUpdate {
    #[ts(type = "string")]
    pub employee_unid: Uuid,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    #[validate(nested)]
    pub values: LeaveRequestValues,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
struct LeaveRequestUpdateRaw {
    pub employee_unid: String,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub values: LeaveRequestValues,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl TryFrom<LeaveRequestUpdateRaw> for LeaveRequestUpdate {
    type Error = AppError;

    fn try_from(raw: LeaveRequestUpdateRaw) -> Result<Self, Self::Error> {
        let employee_unid = Uuid::parse_str(&raw.employee_unid)
            .map_err(|_| AppError::InvalidUuid(raw.employee_unid))?;

        Ok(Self {
            employee_unid,
            leave_type_id: raw.leave_type_id,
            leave_status: raw.leave_status,
            values: raw.values,
            date_requested: raw.date_requested,
            date_finalised: raw.date_finalised,
            notes: raw.notes,
        })
    }
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct AllLeaveRequest {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct MyLeaveRequest {
    #[ts(type = "string")]
    pub employee_unid: Uuid,
    pub leave_type_ids: Vec<i32>,
    pub leave_statuses: Vec<LeaveStatus>,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

struct MyLeaveRequestRaw {
    employee_unid: String,
    leave_type_ids: Vec<i32>,
    leave_statuses: Vec<LeaveStatus>,
    date_from: NaiveDate,
    date_to: NaiveDate,
}

impl TryFrom<MyLeaveRequestRaw> for MyLeaveRequest {
    type Error = AppError;

    fn try_from(raw: MyLeaveRequestRaw) -> Result<Self, Self::Error> {
        let employee_unid = Uuid::parse_str(&raw.employee_unid)
            .map_err(|_| AppError::InvalidUuid(raw.employee_unid))?;

        Ok(Self {
            employee_unid,
            leave_type_ids: raw.leave_type_ids,
            leave_statuses: raw.leave_statuses,
            date_from: raw.date_from,
            date_to: raw.date_to,
        })
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct LeaveRequestStatusUpdate {
    pub leave_status: LeaveStatus,
    pub notes: Option<String>,
}
