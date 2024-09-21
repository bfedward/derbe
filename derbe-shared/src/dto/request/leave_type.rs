use serde::{Deserialize, Serialize};

use crate::deserialization::{deserialize_non_empty_string, deserialize_non_negative_option_f64};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LeaveTypeCreate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
    #[serde(deserialize_with = "deserialize_non_negative_option_f64")]
    pub default_quota: Option<f64>,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}

#[derive(Deserialize)]
pub struct LeaveTypeUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
    #[serde(deserialize_with = "deserialize_non_negative_option_f64")]
    pub default_quota: Option<f64>,
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}
