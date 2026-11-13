use crate::{enums::EmployeeRole, error::AppError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    CreateEmployee,
    ViewEmployees,
    UpdateEmployee,
    DeleteEmployee,
    ApproveLeave,
    RejectLeave,
}

pub trait Permissions {
    fn has_permission(&self, permission: Permission) -> bool;
}

impl Permissions for EmployeeRole {
    fn has_permission(&self, permission: Permission) -> bool {
        match self {
            EmployeeRole::Admin => true,

            EmployeeRole::Manager => matches!(
                permission,
                Permission::CreateEmployee
                    | Permission::ViewEmployees
                    | Permission::UpdateEmployee
                    | Permission::ApproveLeave
                    | Permission::RejectLeave
            ),

            EmployeeRole::RegularUser => matches!(permission, Permission::ViewEmployees),

            EmployeeRole::Demo => matches!(permission, Permission::ViewEmployees),
        }
    }
}

pub struct AccessController;

impl AccessController {
    pub fn authorize<P: Permissions>(
        principal: &P,
        permission: Permission,
    ) -> Result<(), AppError> {
        if principal.has_permission(permission) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}
