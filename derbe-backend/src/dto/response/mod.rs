pub mod leave_request {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    use crate::model::leave_request::LeaveStatus;

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct MyLeaveResponse {
        pub leave_request_id: i32,
        pub leave_type: String,
        pub leave_status: LeaveStatus,
        pub date_from: NaiveDate,
        pub date_to: NaiveDate,
        pub total_duration: f64,
    }
}

