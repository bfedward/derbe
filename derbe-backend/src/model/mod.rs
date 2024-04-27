pub mod leave_type {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct LeaveType {
        pub id: Option<i32>,
        pub name: String,
        pub default_quota: Option<f64>,
        pub can_book_in_future: bool,
        pub show_in_others_days: bool,
        pub default_in_others_days: bool,
        pub show_in_approve_days: bool
    }
}

pub mod division {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Division {
        pub id: Option<i32>,
        pub name: String,
    }
}

pub mod location {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Location {
        pub id: Option<i32>,
        pub name: String,
    }
}

pub mod employee {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Employee {
        pub id: Option<i32>,
        pub first_name: String,
        pub last_name: String,
        pub job_title: String,
        pub division_id: i32,
        pub employment_type: String,
        pub location_id: i32,
        pub manager_id: Option<i32>,
        pub email: String,
        pub phone: String,
    }
}

pub mod leave_request {
    use serde::{Deserialize, Serialize};
    use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
    #[sqlx(type_name = "valid_leave_statuses")] 
    pub enum LeaveStatus {
        PENDING,
        MOREINFOREQ,
        CANCELLED,
        REJECTED,
        ACCEPTED,
    }

    impl PgHasArrayType for LeaveStatus {
        fn array_type_info() -> PgTypeInfo {
            PgTypeInfo::with_name("_valid_leave_statuses")
        }
    }

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    pub struct LeaveRequest {
        pub id: Option<i32>,
        pub employee_id: i32,
        pub leave_type_id: i32,
        pub leave_status: LeaveStatus,
        #[serde_as(as = "Rfc3339")]
        pub date_from: OffsetDateTime,
        #[serde_as(as = "Rfc3339")]
        pub date_to: OffsetDateTime,
        pub first_day_half: bool,
        pub last_day_half: bool,
        #[serde_as(as = "Rfc3339")]
        pub date_requested: OffsetDateTime,
        #[serde_as(as = "Option<Rfc3339>")]
        pub date_finalised: Option<OffsetDateTime>,
        pub notes: Option<String> 
    }
}