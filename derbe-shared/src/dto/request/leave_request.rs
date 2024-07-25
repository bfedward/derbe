use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllLeaveRequest {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}