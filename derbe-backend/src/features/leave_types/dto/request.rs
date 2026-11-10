use serde::Deserialize;
use ts_rs::TS;

use validator::Validate;

#[derive(Deserialize, Validate, Default, Clone, TS)]
#[ts(export)]
pub struct LeaveTypeCreate {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1.0))]
    pub default_quota: Option<f64>,
    #[validate(length(min = 1))]
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}

#[derive(Deserialize, Validate, TS)]
#[ts(export)]
pub struct LeaveTypeUpdate {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1.0))]
    pub default_quota: Option<f64>,
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}
