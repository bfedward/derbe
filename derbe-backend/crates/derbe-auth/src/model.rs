use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

use derbe_core::model::EmployeeRole;

pub struct LoginCredentials {
    pub unid: Uuid,
    pub password: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CurrentUser {
    pub unid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: EmployeeRole,
}
