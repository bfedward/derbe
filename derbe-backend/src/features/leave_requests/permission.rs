use crate::{
    access_control::Permission, enums::EmployeeRole, error::AppError,
    features::auth::middleware::AuthenticatedUser, features::employees::model::EmployeeAccess,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaveRequestPermission {
    List,
    View,
    Create,
    Update,
    Approve,
    Reject,
}

impl Permission for LeaveRequestPermission {
    fn roles_required(self) -> &'static [EmployeeRole] {
        use EmployeeRole::*;

        match self {
            Self::List => &[Admin, Manager, RegularUser, Demo],
            Self::View => &[Admin, Manager, RegularUser],
            // Whose leave requests a role may create or update is decided by its scope.
            Self::Create => &[Admin, Manager, RegularUser],
            Self::Update => &[Admin, Manager, RegularUser],
            Self::Approve => &[Admin, Manager],
            Self::Reject => &[Admin, Manager, RegularUser],
        }
    }
}

/// An admin, or the manager of the employee. Nobody approves their own leave.
pub fn authorize_approve(
    actor: &AuthenticatedUser,
    employee: &EmployeeAccess,
) -> Result<(), AppError> {
    if !LeaveRequestPermission::Approve.is_granted_to(actor.role) {
        return Err(AppError::Forbidden);
    }

    match actor.role {
        EmployeeRole::Admin => Ok(()),
        EmployeeRole::Manager if employee.manager_unid == Some(actor.unid) => Ok(()),
        EmployeeRole::Manager | EmployeeRole::RegularUser | EmployeeRole::Demo => {
            Err(AppError::Forbidden)
        }
    }
}

/// An admin, the manager of the employee, or the employee rejecting their own request.
pub fn authorize_reject(
    actor: &AuthenticatedUser,
    employee: &EmployeeAccess,
) -> Result<(), AppError> {
    if !LeaveRequestPermission::Reject.is_granted_to(actor.role) {
        return Err(AppError::Forbidden);
    }

    match actor.role {
        EmployeeRole::Admin => Ok(()),
        EmployeeRole::Manager if employee.manager_unid == Some(actor.unid) => Ok(()),
        EmployeeRole::RegularUser if employee.unid == actor.unid => Ok(()),
        EmployeeRole::Manager | EmployeeRole::RegularUser | EmployeeRole::Demo => {
            Err(AppError::Forbidden)
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::access_control::authorize_on;

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
    fn roles_per_permission() {
        use EmployeeRole::*;
        use LeaveRequestPermission::*;
        assert_eq!(List.roles_required(), &[Admin, Manager, RegularUser, Demo]);
        assert_eq!(View.roles_required(), &[Admin, Manager, RegularUser]);
        assert_eq!(Create.roles_required(), &[Admin, Manager, RegularUser]);
        assert_eq!(Update.roles_required(), &[Admin, Manager, RegularUser]);
        assert_eq!(Approve.roles_required(), &[Admin, Manager]);
        assert_eq!(Reject.roles_required(), &[Admin, Manager, RegularUser]);
    }

    #[test]
    fn manager_can_create_leave_for_themselves() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(authorize_on(&sam, &sam_as_target, LeaveRequestPermission::Create).is_ok());
    }

    #[test]
    fn manager_can_update_their_own_leave_request() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(authorize_on(&sam, &sam_as_target, LeaveRequestPermission::Update).is_ok());
    }

    #[test]
    fn manager_cannot_approve_their_own_leave() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = as_target(&sam, Some(Uuid::new_v4()));
        assert!(authorize_approve(&sam, &sam_as_target).is_err());
    }

    #[test]
    fn manager_can_approve_and_reject_for_a_report() {
        let sam = user(EmployeeRole::Manager);
        let report = user(EmployeeRole::RegularUser);
        let report_target = as_target(&report, Some(sam.unid));
        assert!(authorize_approve(&sam, &report_target).is_ok());
        assert!(authorize_reject(&sam, &report_target).is_ok());
    }

    #[test]
    fn regular_user_can_reject_only_their_own_leave() {
        let alex = user(EmployeeRole::RegularUser);
        let other = user(EmployeeRole::RegularUser);
        assert!(authorize_reject(&alex, &as_target(&alex, None)).is_ok());
        assert!(authorize_reject(&alex, &as_target(&other, None)).is_err());
        assert!(authorize_approve(&alex, &as_target(&alex, None)).is_err());
    }

    #[test]
    fn manager_cannot_create_leave_for_someone_else_team() {
        let sam = user(EmployeeRole::Manager);
        let stranger = user(EmployeeRole::RegularUser);
        let stranger_target = as_target(&stranger, Some(Uuid::new_v4()));
        assert!(authorize_on(&sam, &stranger_target, LeaveRequestPermission::Create).is_err());
    }
}
