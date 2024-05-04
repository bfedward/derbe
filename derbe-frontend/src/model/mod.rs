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
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Serialize, Deserialize)]
    pub enum LeaveStatus {
        PENDING,
        MOREINFOREQ,
        CANCELLED,
        REJECTED,
        ACCEPTED,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum HalfDayOptions {
        FullDays,
        FirstDayHalf,
        LastDayHalf,
        BothHalfDays,
    }

    // #[derive(Debug, Serialize, Deserialize)]
    // pub struct LeaveRequest {
    //     pub id: Option<i32>,
    //     pub employee_id: i32,
    //     pub leave_type_id: i32,
    //     pub leave_status: LeaveStatus,
    //     #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    //     #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    //     pub date_from: DateTime<Utc>,
    //     #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    //     #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    //     pub date_to: DateTime<Utc>,
    //     pub first_day_half: bool,
    //     pub last_day_half: bool,
    //     #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    //     #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    //     pub date_requested: DateTime<Utc>,
    //     #[serde(serialize_with = "serialize_option_datetime_utc")]
    //     #[serde(deserialize_with = "deserialize_option_datetime_utc")]
    //     pub date_finalised: OptionalDateTimeUtc,
    //     pub notes: Option<String>,
    // }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LeaveRequest {
        pub id: Option<i32>,
        pub employee_id: i32,
        pub leave_type_id: i32,
        pub leave_status: LeaveStatus,
        pub date_from: String,
        pub date_to: String,
        pub first_day_half: bool,
        pub last_day_half: bool,
        pub date_requested: String,
        pub date_finalised: Option<String>,
        pub notes: Option<String>,
    }

    #[derive(Debug)]
    pub struct OptionalDateTimeUtc(pub Option<DateTime<Utc>>);

    impl Serialize for OptionalDateTimeUtc {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match &self.0 {
                Some(date_time) => date_time.serialize(serializer),
                None => serializer.serialize_none(),
            }
        }
    }

    fn serialize_option_datetime_utc<S>(
        value: &OptionalDateTimeUtc,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }

    fn deserialize_option_datetime_utc<'de, D>(
        deserializer: D,
    ) -> Result<OptionalDateTimeUtc, D::Error>
    where
        D: Deserializer<'de>,
    {
        let datetime = DateTime::<Utc>::deserialize(deserializer)?;
        Ok(OptionalDateTimeUtc(Some(datetime)))
    }
}