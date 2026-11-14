use crate::{
    enums::LeaveStatus,
    features::leave_requests::dto::{
        request::{LeaveRequestUpdate, LeaveRequestValues},
        response::LeaveRequestResponse,
    },
};
use chrono::NaiveDate;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Debug, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: i32,
    pub employee_unid: Uuid,
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
            employee_unid: value.employee_unid,
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
            employee_unid: value.employee_unid.to_string(),
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
