use crate::{
    auth::AuthenticatedUser, enums::EmployeeRole, error::AppError,
    features::employees::model::EmployeeAccess,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    CreateDivision,
    ViewDivisions,
    UpdateDivision,
    DeleteDivision,

    CreateLocation,
    ViewLocations,
    UpdateLocation,
    DeleteLocation,

    CreateLeaveType,
    ViewLeaveTypes,
    UpdateLeaveType,
    DeleteLeaveType,

    CreateLeaveRequest,
    ViewLeaveRequests,
    ViewLeaveRequest,
    UpdateLeaveRequest,
    DeleteLeaveRequest,

    CreateEmployee,
    ViewEmployees,
    ViewEmployee,
    UpdateEmployee,
    DeleteEmployee,

    ApproveLeaveRequest,
    RejectLeaveRequest,
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
                Permission::ViewDivisions
                    | Permission::ViewLocations
                    | Permission::ViewLeaveTypes
                    | Permission::CreateLeaveRequest
                    | Permission::ViewLeaveRequest
                    | Permission::UpdateLeaveRequest
                    | Permission::DeleteLeaveRequest
                    | Permission::CreateEmployee
                    | Permission::ViewEmployees
                    | Permission::ViewEmployee
                    | Permission::UpdateEmployee
                    | Permission::ApproveLeaveRequest
                    | Permission::RejectLeaveRequest
            ),

            EmployeeRole::RegularUser => matches!(
                permission,
                Permission::ViewDivisions
                    | Permission::ViewLocations
                    | Permission::ViewLeaveTypes
                    | Permission::ViewEmployees
                    | Permission::CreateLeaveType
                    | Permission::ViewLeaveRequest
                    | Permission::UpdateLeaveRequest
                    | Permission::DeleteLeaveRequest
                    | Permission::RejectLeaveRequest
            ),

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
        if !actor.role.has_permission(Permission::CreateLeaveRequest) {
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
        if !actor.role.has_permission(Permission::ApproveLeaveRequest) {
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
