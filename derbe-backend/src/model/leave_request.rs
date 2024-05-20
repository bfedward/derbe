use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::{deserialization::deserialize_non_negative_f64, DerbeModel};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
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

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub leave_type_id: i32,
    pub leave_status: LeaveStatus,
    #[serde_as(as = "Rfc3339")]
    pub date_from: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub date_to: OffsetDateTime,
    pub first_day_half: bool,
    pub last_day_half: bool,
    #[serde(deserialize_with = "deserialize_non_negative_f64")]
    pub total_duration: f64,
    #[serde_as(as = "Rfc3339")]
    pub date_requested: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    #[serde(default)]
    pub date_finalised: Option<OffsetDateTime>,
    pub notes: Option<String>,
}

impl DerbeModel for LeaveRequest {}
