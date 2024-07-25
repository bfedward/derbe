use std::fmt::Display;

use serde::{Deserialize, Serialize};
use super::{deserialization::deserialize_non_empty_string, DerbeModel};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "valid_employment_types")]
pub enum EmploymentType {
    #[default]
    Permanent,
    Contract,
}


impl Display for EmploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmploymentType::Permanent => write!(f, "Permanent"),
            EmploymentType::Contract => write!(f, "Contract"),
        }
    }
}

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
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl DerbeModel for Employee {}
