use crate::{
    error::AppError,
    features::employees::model::EmployeeRole,
    features::{auth::middleware::AuthenticatedUser, employees::model::EmployeeAccess},
};

/// An action a feature protects. Each feature owns its own enum of permissions
/// and lists, for every permission, the roles that may do it.
pub const ALL_ROLES: &[EmployeeRole] = &[
    EmployeeRole::Admin,
    EmployeeRole::Manager,
    EmployeeRole::RegularUser,
    EmployeeRole::Demo,
];

pub trait Permission: Copy + std::fmt::Debug {
    fn roles_required(self) -> &'static [EmployeeRole];

    fn is_granted_to(self, role: EmployeeRole) -> bool {
        self.roles_required().contains(&role)
    }
}

pub fn authorize(user: &AuthenticatedUser, permission: impl Permission) -> Result<(), AppError> {
    if permission.is_granted_to(user.role) {
        Ok(())
    } else {
        tracing::warn!(user = %user.unid, role = ?user.role, ?permission, "permission denied");
        Err(AppError::Forbidden)
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

/// Like `authorize`, but the role must also cover the employee the action is about.
pub fn authorize_on(
    actor: &AuthenticatedUser,
    target: &EmployeeAccess,
    permission: impl Permission,
) -> Result<(), AppError> {
    if permission.is_granted_to(actor.role) && Scope::of(actor.role).covers(actor, target) {
        Ok(())
    } else {
        tracing::warn!(
            user = %actor.unid,
            role = ?actor.role,
            ?permission,
            target = %target.unid,
            "permission denied"
        );
        Err(AppError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[derive(Clone, Copy, Debug)]
    struct AnyoneCan;

    impl Permission for AnyoneCan {
        fn roles_required(self) -> &'static [EmployeeRole] {
            ALL_ROLES
        }
    }

    fn user(role: EmployeeRole) -> AuthenticatedUser {
        AuthenticatedUser {
            unid: Uuid::new_v4(),
            role,
        }
    }

    fn target(unid: Uuid, manager_unid: Option<Uuid>) -> EmployeeAccess {
        EmployeeAccess { unid, manager_unid }
    }

    #[test]
    fn admin_covers_everyone() {
        let admin = user(EmployeeRole::Admin);
        assert!(authorize_on(&admin, &target(Uuid::new_v4(), None), AnyoneCan).is_ok());
    }

    #[test]
    fn manager_covers_themselves_and_their_reports_only() {
        let manager = user(EmployeeRole::Manager);
        assert!(authorize_on(&manager, &target(manager.unid, None), AnyoneCan).is_ok());
        assert!(authorize_on(
            &manager,
            &target(Uuid::new_v4(), Some(manager.unid)),
            AnyoneCan
        )
        .is_ok());
        assert!(authorize_on(
            &manager,
            &target(Uuid::new_v4(), Some(Uuid::new_v4())),
            AnyoneCan
        )
        .is_err());
    }

    #[test]
    fn regular_user_covers_only_themselves() {
        let regular = user(EmployeeRole::RegularUser);
        assert!(authorize_on(&regular, &target(regular.unid, None), AnyoneCan).is_ok());
        assert!(authorize_on(
            &regular,
            &target(Uuid::new_v4(), Some(regular.unid)),
            AnyoneCan
        )
        .is_err());
    }

    #[test]
    fn demo_covers_nobody() {
        let demo = user(EmployeeRole::Demo);
        assert!(authorize_on(&demo, &target(demo.unid, None), AnyoneCan).is_err());
    }

    #[test]
    fn scope_never_widens_a_role_that_lacks_the_permission() {
        #[derive(Clone, Copy, Debug)]
        struct AdminOnly;

        impl Permission for AdminOnly {
            fn roles_required(self) -> &'static [EmployeeRole] {
                &[EmployeeRole::Admin]
            }
        }

        let manager = user(EmployeeRole::Manager);
        assert!(authorize_on(&manager, &target(manager.unid, None), AdminOnly).is_err());
    }
}
