use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::types::chrono::{DateTime, Utc};

use super::{deserialization::deserialize_non_negative_f64, DerbeModel};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "valid_leave_statuses")]
pub enum LeaveStatus {
    Pending,
    MoreInfoReq,
    Cancelled,
    Rejected,
    Accepted,
}

impl PgHasArrayType for LeaveStatus {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_valid_leave_statuses")
    }
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

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub first_day_half: bool,
    pub last_day_half: bool,
    #[serde(deserialize_with = "deserialize_non_negative_f64")]
    pub total_duration: f64,
    pub date_requested: DateTime<Utc>,
    pub date_finalised: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

impl DerbeModel for LeaveRequest {}
