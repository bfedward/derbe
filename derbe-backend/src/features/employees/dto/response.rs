use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::enums::EmploymentType;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EmployeeResponse {
    pub unid: String,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_unid: Option<String>,
    pub email: String,
}

impl EmployeeResponse {
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EmployeeDetailsResponse {
    pub unid: String,
    pub name: String,
    pub job_title: String,
    pub division: String,
    pub employment_type: String,
    pub location: String,
    pub manager: Option<String>,
    pub email: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreatedEmployeeResponse {
    pub employee: EmployeeResponse,
    pub temporary_password: String,
}
