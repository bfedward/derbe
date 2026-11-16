use crate::{enums::EmployeeRole, error::AppError, features::auth::middleware::AuthenticatedUser};

/// An action a feature protects. Each feature owns its own enum of permissions
/// and lists, for every permission, the roles that may do it.
pub trait Permission: Copy {
    fn roles_required(self) -> &'static [EmployeeRole];

    fn is_granted_to(self, role: EmployeeRole) -> bool {
        self.roles_required().contains(&role)
    }
}

pub fn authorize(user: &AuthenticatedUser, permission: impl Permission) -> Result<(), AppError> {
    if permission.is_granted_to(user.role) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
