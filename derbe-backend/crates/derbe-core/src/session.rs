use uuid::Uuid;

use crate::model::EmployeeRole;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub unid: Uuid,
    pub role: EmployeeRole,
}
