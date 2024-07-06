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
}

pub mod employee {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct EmployeeDetailsResponse {
        pub id: i32,
        pub name: String,
        pub job_title: String,
        pub division: String,
        pub employment_type: String,
        pub location: String,
        pub manager: Option<String>,
        pub email: String,
        pub phone: Option<String>,
    }
}
