use crate::{access_control::Permission, enums::EmployeeRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationPermission {
    Create,
    View,
}

impl Permission for LocationPermission {
    fn roles_required(self) -> &'static [EmployeeRole] {
        use EmployeeRole::*;

        match self {
            Self::Create => &[Admin],
            Self::View => &[Admin, Manager, RegularUser, Demo],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_admin_can_create() {
        assert_eq!(
            LocationPermission::Create.roles_required(),
            &[EmployeeRole::Admin]
        );
    }

    #[test]
    fn every_role_can_view() {
        for role in [
            EmployeeRole::Admin,
            EmployeeRole::Manager,
            EmployeeRole::RegularUser,
            EmployeeRole::Demo,
        ] {
            assert!(LocationPermission::View.is_granted_to(role));
        }
    }
}
