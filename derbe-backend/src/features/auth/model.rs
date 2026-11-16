use uuid::Uuid;

use crate::{features::auth::dto::MeResponse, features::employees::model::EmployeeRole};

pub struct LoginCredentials {
    pub unid: Uuid,
    pub password: String,
}

pub struct CurrentUser {
    pub unid: Uuid,
    pub email: String,
    pub role: EmployeeRole,
}

impl From<CurrentUser> for MeResponse {
    fn from(value: CurrentUser) -> Self {
        Self {
            unid: value.unid,
            email: value.email,
            role: value.role,
        }
    }
}
