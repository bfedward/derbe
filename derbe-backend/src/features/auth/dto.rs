use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::EmployeeRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub unid: Uuid,
    pub email: String,
    pub role: EmployeeRole,
}
