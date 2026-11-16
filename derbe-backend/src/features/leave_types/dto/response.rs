use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::features::leave_types::model::OthersDaysVisibility;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveTypeResponse {
    pub id: i32,
    pub name: String,
    pub default_quota: Option<f64>,
    pub colour: String,
    pub can_book_in_future: bool,
    pub others_days_visibility: OthersDaysVisibility,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}
