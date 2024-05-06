pub mod leave_request {
    use serde::{Deserialize, Serialize};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    use crate::model::leave_request::LeaveStatus;

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct MyLeaveRequest {
        pub employee_id: i32,
        pub leave_type_ids: Vec<i32>,
        pub leave_statuses: Vec<LeaveStatus>,
        #[serde_as(as = "Rfc3339")]
        pub date_from: OffsetDateTime,
        #[serde_as(as = "Rfc3339")]
        pub date_to: OffsetDateTime,
    }
}