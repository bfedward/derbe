pub mod leave_request {
    use serde::{Deserialize, Serialize};
    use sqlx::types::chrono::{DateTime, Utc};

    use crate::model::leave_request::LeaveStatus;

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct MyLeaveResponse {
        pub leave_request_id: i32,
        pub leave_type: String,
        pub leave_status: LeaveStatus,
        pub date_from: DateTime<Utc>,
        pub date_to: DateTime<Utc>,
        pub total_duration: f64,
    }
}
