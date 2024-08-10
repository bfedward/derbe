use chrono::NaiveDate;
use derbe_shared::{
    dto::{
        request::leave_request::{LeaveRequestUpdate, LeaveRequestValues},
        response::leave_request::LeaveRequestResponse,
    },
    enums::LeaveStatus,
};
use sqlx::types::chrono::{DateTime, Utc};

#[serde_with::serde_as]
#[derive(Debug, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: i32,
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

impl From<LeaveRequest> for LeaveRequestUpdate {
    fn from(value: LeaveRequest) -> Self {
        Self {
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

impl From<LeaveRequest> for LeaveRequestResponse {
    fn from(value: LeaveRequest) -> Self {
        Self {
            id: value.id,
            employee_id: value.employee_id,
            leave_type_id: value.leave_type_id,
            leave_status: value.leave_status,
            date_from: value.date_from,
            date_to: value.date_to,
            first_day_half: value.first_day_half,
            last_day_half: value.last_day_half,
            total_duration: value.total_duration,
            date_requested: value.date_requested,
            date_finalised: value.date_finalised,
            notes: value.notes,
        }
    }
}
