use crate::{
    access_control::{Permission, ALL_ROLES},
    features::employees::model::EmployeeRole,
};

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
}
