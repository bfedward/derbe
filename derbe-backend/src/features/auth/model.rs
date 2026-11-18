use serde::Serialize;
use uuid::Uuid;

use crate::features::employees::model::EmployeeRole;

pub struct LoginCredentials {
    pub unid: Uuid,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CurrentUser {
    pub unid: Uuid,
    pub email: String,
    pub role: EmployeeRole,
}
