use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    enums::{EmployeeRole, EmploymentType},
    features::employees::dto::response::{CreatedEmployeeResponse, EmployeeResponse},
};

pub struct Employee {
    pub unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub email: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_unid: Option<Uuid>,
}

impl From<Employee> for EmployeeResponse {
    fn from(value: Employee) -> Self {
        Self {
            unid: value.unid.to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            job_title: value.job_title,
            email: value.email,
            division_id: value.division_id,
            employment_type: value.employment_type,
            location_id: value.location_id,
            manager_unid: value.manager_unid.map(|m| m.to_string()),
        }
    }
}

pub struct EmployeeRecord {
    pub unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub email: String,
    pub password: String,
    pub role: EmployeeRole,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_unid: Option<Uuid>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub email_change_pending: Option<String>,
    pub email_change_token: Option<String>,
    pub email_change_token_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreatedEmployee {
    pub employee: Employee,
    pub temporary_password: String,
}

impl From<CreatedEmployee> for CreatedEmployeeResponse {
    fn from(value: CreatedEmployee) -> Self {
        let employee: EmployeeResponse = value.employee.into();

        Self {
            employee,
            temporary_password: value.temporary_password,
        }
    }
}

pub struct EmployeeAccess {
    pub unid: Uuid,
    pub manager_unid: Option<Uuid>,
}