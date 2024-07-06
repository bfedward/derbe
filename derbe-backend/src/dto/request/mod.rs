pub mod leave_request {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    use crate::model::leave_request::LeaveStatus;

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct MyLeaveRequest {
        pub employee_id: i32,
        pub leave_type_ids: Vec<i32>,
        pub leave_statuses: Vec<LeaveStatus>,
        pub date_from: NaiveDate,
        pub date_to: NaiveDate,
    }

    #[derive(Deserialize)]
    pub struct AllLeaveRequest {
        pub date_from: NaiveDate,
        pub date_to: NaiveDate,
    }

    #[derive(Debug, Deserialize)]
    pub struct LeaveRequestStatusUpdate {
        pub leave_status: LeaveStatus,
        pub notes: Option<String>,
    }
}
