use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllLeaveResponse {
    pub employee_name: String,
    pub leaves: Vec<AllLeaveInnerResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AllLeaveInnerResponse {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub first_day_half: bool,
    pub last_day_half: bool,
    pub colour: String,
}