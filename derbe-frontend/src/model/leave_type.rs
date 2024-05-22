use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct LeaveType {
    pub id: Option<i32>,
    pub name: String,
    pub default_quota: Option<f64>,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}
