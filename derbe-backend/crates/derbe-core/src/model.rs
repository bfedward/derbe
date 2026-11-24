use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;
use uuid::Uuid;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sqlx::Type,
    TS,
    Display,
    EnumString,
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

pub struct EmployeeAccess {
    pub unid: Uuid,
    pub manager_unid: Option<Uuid>,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sqlx::Type,
    TS,
    Display,
    EnumString,
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

impl LeaveStatus {
    /// A decided request (`Rejected`) is final, and `Accepted` can only be cancelled.
    pub const fn can_become(self, next: Self) -> bool {
        use LeaveStatus::{Accepted, Cancelled, MoreInfoReq, Pending, Rejected};
        matches!(
            (self, next),
            (Pending, Accepted | Rejected | Cancelled | MoreInfoReq)
                | (MoreInfoReq, Pending | Accepted | Rejected | Cancelled)
                | (Accepted, Cancelled)
        )
    }
}
