use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::features::employees::model::EmploymentType;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EmployeeResponse {
    #[ts(type = "string")]
    pub employee_unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    #[ts(type = "string | null")]
    pub manager_unid: Option<Uuid>,
    pub email: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EmployeeDetailsResponse {
    pub employee_unid: String,
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
