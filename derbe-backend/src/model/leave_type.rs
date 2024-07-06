use serde::{Deserialize, Serialize};

use super::{
    deserialization::{deserialize_non_empty_string, deserialize_non_negative_option_f64},
    DerbeModel,
};

#[derive(Serialize, Deserialize)]
pub struct LeaveType {
    pub id: Option<i32>,
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

impl DerbeModel for LeaveType {}
