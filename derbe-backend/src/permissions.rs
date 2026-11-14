use crate::{auth::AuthenticatedUser, enums::EmployeeRole, error::AppError, features::employees::model::{Employee, EmployeeAccess}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    CreateLeave,
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

    pub fn authorize_create_leave(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(Permission::CreateLeave) {
            return Err(AppError::Forbidden);
        }

        match actor.role {
            EmployeeRole::Admin => Ok(()),

            EmployeeRole::Manager => {
                if target.manager_unid == Some(actor.unid) {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            EmployeeRole::RegularUser => {
                if target.unid == actor.unid {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            EmployeeRole::Demo => Err(AppError::Forbidden),
        }
    }

    pub fn authorize_approve_leave(
        actor: &AuthenticatedUser,
        employee: &EmployeeAccess,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(Permission::ApproveLeave) {
            return Err(AppError::Forbidden);
        }

        match actor.role {
            EmployeeRole::Admin => Ok(()),

            EmployeeRole::Manager => {
                if employee.manager_unid == Some(actor.unid) {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            _ => Err(AppError::Forbidden),
        }
    }
}
