use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, TS, Display,
)]
#[sqlx(type_name = "valid_employment_types")]
#[ts(export)]
pub enum EmploymentType {
    #[default]
    Permanent,
    Contract,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, TS, Display,
)]
#[sqlx(type_name = "valid_leave_statuses")]
#[ts(export)]
pub enum LeaveStatus {
    #[default]
    Pending,
    MoreInfoReq,
    Cancelled,
    Rejected,
    Accepted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS, EnumString)]
#[ts(export)]
pub enum HalfDayOptions {
    #[default]
    #[strum(serialize = "allDaysFull")]
    FullDays,
    #[strum(serialize = "firstDayHalf")]
    FirstDayHalf,
    #[strum(serialize = "lastDayHalf")]
    LastDayHalf,
    #[strum(serialize = "firstAndLastHalfs")]
    BothHalfDays,
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

#[cfg(test)]
mod tests {
    use super::{EmploymentType, HalfDayOptions, LeaveStatus};

    #[test]
    fn enum_display_keeps_existing_api_labels() {
        assert_eq!(EmploymentType::Permanent.to_string(), "Permanent");
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
