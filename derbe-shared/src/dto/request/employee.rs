use serde::{Deserialize, Serialize};

use crate::{
    deserialization::deserialize_non_empty_string, dto::response::employee::EmployeeResponse,
    enums::EmploymentType,
};

#[derive(Serialize, Clone, Deserialize, Default)]
pub struct EmployeeCreate {
    pub id: Option<i32>,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub first_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmployeeUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub first_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl From<EmployeeResponse> for EmployeeUpdate {
    fn from(value: EmployeeResponse) -> Self {
        EmployeeUpdate {
            first_name: value.first_name,
            last_name: value.last_name,
            job_title: value.job_title,
            division_id: value.division_id,
            employment_type: value.employment_type,
            location_id: value.location_id,
            manager_id: value.manager_id,
            email: value.email,
            phone: value.phone,
        }
    }
}
