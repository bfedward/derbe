use crate::dto::{
    request::{LeaveRequestUpdate, LeaveRequestValues},
    response::LeaveRequestResponse,
};
use chrono::NaiveDate;
use derbe_core::model::LeaveStatus;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use strum::EnumString;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS, EnumString)]
#[strum(serialize_all = "camelCase")]
#[ts(export)]
pub enum HalfDayOptions {
    #[default]
    #[strum(serialize = "allDaysFull")]
    FullDays,
    FirstDayHalf,
    LastDayHalf,
    #[strum(serialize = "firstAndLastHalfs")]
    BothHalfDays,
}

pub struct LeaveRequestNames {
    pub employee_name: String,
    pub leave_type: String,
}

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
            values: LeaveRequestValues {
                date_from: value.date_from,
                date_to: value.date_to,
                first_day_half: value.first_day_half,
                last_day_half: value.last_day_half,
                total_duration: value.total_duration,
            },
            date_requested: value.date_requested,
            notes: value.notes,
        }
    }
}

impl From<LeaveRequest> for LeaveRequestResponse {
    fn from(value: LeaveRequest) -> Self {
        Self {
            id: value.id,
            employee_unid: value.employee_unid,
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

impl HalfDayOptions {
    pub const fn from_bools(half_days: (bool, bool)) -> Self {
        match half_days {
            (true, true) => Self::BothHalfDays,
            (true, false) => Self::FirstDayHalf,
            (false, true) => Self::LastDayHalf,
            (false, false) => Self::FullDays,
        }
    }

    pub const fn to_bools(&self) -> (bool, bool) {
        match self {
            Self::FullDays => (false, false),
            Self::FirstDayHalf => (true, false),
            Self::LastDayHalf => (false, true),
            Self::BothHalfDays => (true, true),
        }
    }

    pub const fn to_f64(&self) -> f64 {
        match self {
            Self::FullDays => 0.0,
            Self::FirstDayHalf | Self::LastDayHalf => 0.5,
            Self::BothHalfDays => 1.0,
        }
    }

    pub fn from_string(value: &str) -> Self {
        Self::try_from_string(value).unwrap_or_else(|_| panic!("Unknown HalfDayOption: {value}"))
    }

    pub fn try_from_string(value: &str) -> Result<Self, strum::ParseError> {
        value.parse()
    }
}

pub struct EmailDetails {
    pub employee_name: String,
    pub employee_email: String,
    pub leave_type: String,
    pub manager_unid: Option<Uuid>,
    pub manager_email: Option<String>,
    pub actor_name: String,
}

#[cfg(test)]
mod tests {
    use super::HalfDayOptions;
    use derbe_core::model::LeaveStatus;

    #[test]
    fn a_decided_request_cannot_go_back_to_pending() {
        assert!(!LeaveStatus::Rejected.can_become(LeaveStatus::Pending));
        assert!(!LeaveStatus::Accepted.can_become(LeaveStatus::Pending));
        assert!(!LeaveStatus::Cancelled.can_become(LeaveStatus::Pending));
    }

    #[test]
    fn a_pending_request_can_be_decided() {
        assert!(LeaveStatus::Pending.can_become(LeaveStatus::Accepted));
        assert!(LeaveStatus::Pending.can_become(LeaveStatus::Rejected));
        assert!(LeaveStatus::Pending.can_become(LeaveStatus::MoreInfoReq));
    }

    #[test]
    fn a_request_waiting_for_info_can_be_decided_or_resubmitted() {
        assert!(LeaveStatus::MoreInfoReq.can_become(LeaveStatus::Pending));
        assert!(LeaveStatus::MoreInfoReq.can_become(LeaveStatus::Accepted));
        assert!(LeaveStatus::MoreInfoReq.can_become(LeaveStatus::Rejected));
    }

    #[test]
    fn an_accepted_request_can_only_be_cancelled() {
        assert!(LeaveStatus::Accepted.can_become(LeaveStatus::Cancelled));
        assert!(!LeaveStatus::Accepted.can_become(LeaveStatus::Rejected));
    }

    #[test]
    fn leave_status_display_keeps_existing_api_labels() {
        assert_eq!(LeaveStatus::MoreInfoReq.to_string(), "MoreInfoReq");
    }

    #[test]
    fn half_day_options_parse_existing_values_without_panicking() {
        assert_eq!(
            HalfDayOptions::try_from_string("allDaysFull"),
            Ok(HalfDayOptions::FullDays)
        );
        assert_eq!(
            HalfDayOptions::try_from_string("firstDayHalf"),
            Ok(HalfDayOptions::FirstDayHalf)
        );
        assert_eq!(
            HalfDayOptions::try_from_string("lastDayHalf"),
            Ok(HalfDayOptions::LastDayHalf)
        );
        assert_eq!(
            HalfDayOptions::try_from_string("firstAndLastHalfs"),
            Ok(HalfDayOptions::BothHalfDays)
        );
        assert!(HalfDayOptions::try_from_string("unknown").is_err());
    }
}
