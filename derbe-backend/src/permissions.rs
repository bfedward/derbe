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
                    | Permission::ViewLeaveRequests
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
                    | Permission::UpdateEmployee
                    | Permission::CreateLeaveRequest
                    | Permission::ViewLeaveRequest
                    | Permission::ViewLeaveRequests
                    | Permission::UpdateLeaveRequest
                    | Permission::DeleteLeaveRequest
                    | Permission::RejectLeaveRequest
            ),

            EmployeeRole::Demo => matches!(
                permission,
                Permission::ViewDivisions
                    | Permission::ViewLocations
                    | Permission::ViewLeaveTypes
                    | Permission::ViewLeaveRequests
                    | Permission::ViewEmployees
            ),
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

    pub fn authorize_get_leave_request(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_leave_request_core(actor, target, Permission::ViewLeaveRequest)
    }

    pub fn authorize_get_my_leave_requests(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_leave_request_core(actor, target, Permission::ViewLeaveRequest)
    }

    pub fn authorize_create_leave_request(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_leave_request_core(actor, target, Permission::CreateLeaveRequest)
    }

    pub fn authorize_update_leave_request(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_leave_request_core(actor, target, Permission::UpdateLeaveRequest)
    }

    fn authorize_leave_request_core(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
        permission: Permission,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(permission) {
            return Err(AppError::Forbidden);
        }

        match actor.role {
            EmployeeRole::Admin => Ok(()),

            EmployeeRole::Manager => {
                if target.unid == actor.unid || target.manager_unid == Some(actor.unid) {
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

    pub fn authorize_approve_leave_request(
        actor: &AuthenticatedUser,
        employee: &EmployeeAccess,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(Permission::ApproveLeaveRequest) {
            return Err(AppError::Forbidden);
        }

        match actor.role {
            EmployeeRole::Admin => Ok(()),

            // Managers can approve their employee's leave requests
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

    pub fn authorize_reject_leave_request(
        actor: &AuthenticatedUser,
        employee: &EmployeeAccess,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(Permission::RejectLeaveRequest) {
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

            // RegularUser can reject their own leave requests
            EmployeeRole::RegularUser => {
                if actor.unid == employee.unid {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            _ => Err(AppError::Forbidden),
        }
    }

    pub fn authorize_update_employee(
        actor: &AuthenticatedUser,
        employee: &EmployeeAccess,
    ) -> Result<(), AppError> {
        if !actor.role.has_permission(Permission::UpdateEmployee) {
            return Err(AppError::Forbidden);
        }

        match actor.role {
            EmployeeRole::Admin => Ok(()),

            EmployeeRole::Manager => {
                if employee.unid == actor.unid || employee.manager_unid == Some(actor.unid) {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            // RegularUser can update themselves
            EmployeeRole::RegularUser => {
                if actor.unid == employee.unid {
                    Ok(())
                } else {
                    Err(AppError::Forbidden)
                }
            }

            _ => Err(AppError::Forbidden),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn user(role: EmployeeRole) -> AuthenticatedUser {
        AuthenticatedUser {
            unid: Uuid::new_v4(),
            role,
        }
    }

    fn as_target(user: &AuthenticatedUser, manager_unid: Option<Uuid>) -> EmployeeAccess {
        EmployeeAccess {
            unid: user.unid,
            manager_unid,
        }
    }

    #[test]
    fn manager_can_create_leave_for_themselves() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(AccessController::authorize_create_leave_request(&sam, &sam_as_target).is_ok());
    }

    #[test]
    fn manager_can_update_their_own_leave_request() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(AccessController::authorize_update_leave_request(&sam, &sam_as_target).is_ok());
    }

    #[test]
    fn manager_can_update_themselves() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(AccessController::authorize_update_employee(&sam, &sam_as_target).is_ok());
    }

    #[test]
    fn manager_cannot_approve_their_own_leave() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(AccessController::authorize_approve_leave_request(&sam, &sam_as_target).is_err());
    }

    #[test]
    fn manager_cannot_create_leave_for_someone_else_team() {
        let sam = user(EmployeeRole::Manager);
        let stranger = user(EmployeeRole::RegularUser);
        let stranger_target = as_target(&stranger, Some(Uuid::new_v4()));
        assert!(AccessController::authorize_create_leave_request(&sam, &stranger_target).is_err());
    }
}
