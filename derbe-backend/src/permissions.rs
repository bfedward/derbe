use crate::{
    auth::AuthenticatedUser, enums::EmployeeRole, error::AppError,
    features::employees::model::EmployeeAccess,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    CreateDivision,
    ViewDivisions,

    CreateLocation,
    ViewLocations,

    CreateLeaveType,
    ViewLeaveTypes,

    CreateLeaveRequest,
    ViewLeaveRequests,
    ViewLeaveRequest,
    UpdateLeaveRequest,

    CreateEmployee,
    ViewEmployees,
    ViewEmployee,
    UpdateEmployee,

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

/// Whose records a role may act on.
#[derive(Clone, Copy)]
enum Scope {
    Everyone,
    SelfAndReports,
    SelfOnly,
    Nobody,
}

impl Scope {
    fn of(role: EmployeeRole) -> Self {
        match role {
            EmployeeRole::Admin => Self::Everyone,
            EmployeeRole::Manager => Self::SelfAndReports,
            EmployeeRole::RegularUser => Self::SelfOnly,
            EmployeeRole::Demo => Self::Nobody,
        }
    }

    fn covers(self, actor: &AuthenticatedUser, target: &EmployeeAccess) -> bool {
        match self {
            Self::Everyone => true,
            Self::SelfAndReports => {
                target.unid == actor.unid || target.manager_unid == Some(actor.unid)
            }
            Self::SelfOnly => target.unid == actor.unid,
            Self::Nobody => false,
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

    pub fn authorize_create_leave_request(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_on(actor, target, Permission::CreateLeaveRequest)
    }

    pub fn authorize_update_leave_request(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
    ) -> Result<(), AppError> {
        Self::authorize_on(actor, target, Permission::UpdateLeaveRequest)
    }

    fn authorize_on(
        actor: &AuthenticatedUser,
        target: &EmployeeAccess,
        permission: Permission,
    ) -> Result<(), AppError> {
        if actor.role.has_permission(permission) && Scope::of(actor.role).covers(actor, target) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
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
        Self::authorize_on(actor, employee, Permission::UpdateEmployee)
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

    const ALL_PERMISSIONS: [Permission; 16] = [
        Permission::CreateDivision,
        Permission::ViewDivisions,
        Permission::CreateLocation,
        Permission::ViewLocations,
        Permission::CreateLeaveType,
        Permission::ViewLeaveTypes,
        Permission::CreateLeaveRequest,
        Permission::ViewLeaveRequests,
        Permission::ViewLeaveRequest,
        Permission::UpdateLeaveRequest,
        Permission::CreateEmployee,
        Permission::ViewEmployees,
        Permission::ViewEmployee,
        Permission::UpdateEmployee,
        Permission::ApproveLeaveRequest,
        Permission::RejectLeaveRequest,
    ];

    fn granted(role: EmployeeRole) -> Vec<Permission> {
        ALL_PERMISSIONS
            .into_iter()
            .filter(|permission| role.has_permission(*permission))
            .collect()
    }

    #[test]
    fn admin_has_every_permission() {
        assert_eq!(granted(EmployeeRole::Admin), ALL_PERMISSIONS.to_vec());
    }

    #[test]
    fn manager_permissions() {
        use Permission::*;
        assert_eq!(
            granted(EmployeeRole::Manager),
            vec![
                ViewDivisions,
                ViewLocations,
                ViewLeaveTypes,
                CreateLeaveRequest,
                ViewLeaveRequests,
                ViewLeaveRequest,
                UpdateLeaveRequest,
                CreateEmployee,
                ViewEmployees,
                ViewEmployee,
                UpdateEmployee,
                ApproveLeaveRequest,
                RejectLeaveRequest,
            ]
        );
    }

    #[test]
    fn regular_user_permissions() {
        use Permission::*;
        assert_eq!(
            granted(EmployeeRole::RegularUser),
            vec![
                ViewDivisions,
                ViewLocations,
                ViewLeaveTypes,
                CreateLeaveRequest,
                ViewLeaveRequests,
                ViewLeaveRequest,
                UpdateLeaveRequest,
                ViewEmployees,
                UpdateEmployee,
                RejectLeaveRequest,
            ]
        );
    }

    #[test]
    fn demo_can_only_read() {
        use Permission::*;
        assert_eq!(
            granted(EmployeeRole::Demo),
            vec![
                ViewDivisions,
                ViewLocations,
                ViewLeaveTypes,
                ViewLeaveRequests,
                ViewEmployees,
            ]
        );
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
