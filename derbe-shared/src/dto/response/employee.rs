use serde::{Deserialize, Serialize};

use crate::enums::EmploymentType;

#[derive(Serialize, Deserialize)]
pub struct EmployeeResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl EmployeeResponse {
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

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
