use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;
use uuid::Uuid;

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
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    sqlx::Type,
    TS,
    EnumString,
    Display,
)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
pub enum EmployeeRole {
    Admin,
    Manager,
    #[default]
    RegularUser,
    Demo,
}

pub struct EmployeeRecord {
    pub unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub email: String,
    pub password: String,
    pub role: EmployeeRole,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_unid: Option<Uuid>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub email_change_pending: Option<String>,
    pub email_change_token: Option<String>,
    pub email_change_token_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct EmployeeAccess {
    pub unid: Uuid,
    pub manager_unid: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::EmploymentType;

    #[test]
    fn employment_type_display_keeps_existing_api_labels() {
        assert_eq!(EmploymentType::Permanent.to_string(), "Permanent");
        assert_eq!(EmploymentType::Contract.to_string(), "Contract");
    }
}
