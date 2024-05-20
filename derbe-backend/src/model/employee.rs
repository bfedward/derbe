use serde::{Deserialize, Serialize};

use super::{deserialization::deserialize_non_empty_string, DerbeModel};

#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i32>,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub first_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub job_title: String,
    pub division_id: i32,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub employment_type: String,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl DerbeModel for Employee {}
