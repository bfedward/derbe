use crate::dto::request::EmployeeCreate;
use derbe_core::{
    access_control::{ALL_ROLES, Permission, authorize},
    error::AppError,
    model::EmployeeRole,
    session::AuthenticatedUser,
};
use tracing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmployeePermission {
    List,
    View,
    Create,
    Update,
}

impl Permission for EmployeePermission {
    fn roles_required(self) -> &'static [EmployeeRole] {
        use EmployeeRole::*;

        match self {
            Self::List => ALL_ROLES,
            Self::View => &[Admin, Manager],
            Self::Create => &[Admin, Manager],
            // Which employees a role may update is decided by its scope.
            Self::Update => &[Admin, Manager, RegularUser],
        }
    }
}

fn assignable_roles(actor: EmployeeRole) -> &'static [EmployeeRole] {
    use EmployeeRole::*;
    match actor {
        Admin => &[Manager, RegularUser],
        Manager => &[RegularUser],
        RegularUser | Demo => &[],
    }
}

pub fn authorize_create(actor: &AuthenticatedUser, new: &EmployeeCreate) -> Result<(), AppError> {
    authorize(actor, EmployeePermission::Create)?;

    if !assignable_roles(actor.role).contains(&new.role) {
        tracing::warn!(user = %actor.unid, role = ?actor.role, new_role = ?new.role, "role not assignable");
        return Err(AppError::Forbidden);
    }

    if actor.role == EmployeeRole::Manager && new.manager_unid != Some(actor.unid) {
        tracing::warn!(user = %actor.unid, "manager creating an employee outside their team");
        return Err(AppError::Forbidden);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles_per_permission() {
        use EmployeeRole::*;
        assert_eq!(EmployeePermission::List.roles_required(), ALL_ROLES);
        assert_eq!(EmployeePermission::View.roles_required(), &[Admin, Manager]);
        assert_eq!(
            EmployeePermission::Create.roles_required(),
            &[Admin, Manager]
        );
        assert_eq!(
            EmployeePermission::Update.roles_required(),
            &[Admin, Manager, RegularUser]
        );
    }

    #[test]
    fn assignable_roles_per_creator() {
        use EmployeeRole::*;
        assert_eq!(assignable_roles(Admin), &[Manager, RegularUser]);
        assert_eq!(assignable_roles(Manager), &[RegularUser]);
        assert!(assignable_roles(RegularUser).is_empty());
        assert!(assignable_roles(Demo).is_empty());
    }

    #[test]
    fn nobody_can_create_an_admin_or_a_demo_user() {
        use EmployeeRole::*;
        for actor in [Admin, Manager, RegularUser, Demo] {
            assert!(!assignable_roles(actor).contains(&Admin));
            assert!(!assignable_roles(actor).contains(&Demo));
        }
    }
}
