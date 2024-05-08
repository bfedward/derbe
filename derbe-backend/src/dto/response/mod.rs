pub mod leave_request {
    use serde::{Deserialize, Serialize};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    use crate::model::leave_request::LeaveStatus;

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct MyLeaveResponse {
        pub leave_request_id: i32,
        pub leave_type: String,
        pub leave_status: LeaveStatus,
        #[serde_as(as = "Rfc3339")]
        pub date_from: OffsetDateTime,
        #[serde_as(as = "Rfc3339")]
        pub date_to: OffsetDateTime,
        pub total_duration: f64,
    }
}
