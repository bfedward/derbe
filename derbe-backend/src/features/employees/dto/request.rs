use serde::Deserialize;
use ts_rs::TS;

use crate::{enums::EmploymentType, features::employees::dto::response::EmployeeResponse};

use validator::Validate;

#[derive(Validate, Clone, Deserialize, Default, TS)]
#[ts(export)]
pub struct EmployeeCreate {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 1))]
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Validate, Deserialize, Clone, TS)]
#[ts(export)]
pub struct EmployeeUpdate {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 1))]
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
