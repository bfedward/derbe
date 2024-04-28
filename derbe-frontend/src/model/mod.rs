pub mod leave_type {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
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

pub mod leave_request {
    use serde::{Deserialize, Serialize};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    #[derive(Debug, Serialize, Deserialize)]
    pub enum LeaveStatus {
        PENDING,
        MOREINFOREQ,
        CANCELLED,
        REJECTED,
        ACCEPTED,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum HalfDayOptions {
        FullDays,
        FirstDayHalf,
        LastDayHalf,
        BothHalfDays
    }

    #[serde_with::serde_as]
    #[derive(Debug, Serialize, Deserialize)]
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