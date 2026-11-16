use crate::{
    access_control::{Permission, ALL_ROLES},
    features::employees::model::EmployeeRole,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivisionPermission {
    Create,
    View,
}

impl Permission for DivisionPermission {
    fn roles_required(self) -> &'static [EmployeeRole] {
        use EmployeeRole::*;

        match self {
            Self::Create => &[Admin],
            Self::View => ALL_ROLES,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_admin_can_create() {
        assert_eq!(
            DivisionPermission::Create.roles_required(),
            &[EmployeeRole::Admin]
        );
    }

    #[test]
    fn every_role_can_view() {
        for role in ALL_ROLES.iter().copied() {
            assert!(DivisionPermission::View.is_granted_to(role));
        }
    }
}
