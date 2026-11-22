use uuid::Uuid;

use crate::{
    error::AppError,
    features::auth::model::{CurrentUser, LoginCredentials},
    features::employees::model::EmployeeRole,
    http::AppState,
};

pub async fn find_login_by_email(
    email: &str,
    ctx: &AppState,
) -> Result<Option<LoginCredentials>, AppError> {
    Ok(sqlx::query_as!(
        LoginCredentials,
        "SELECT unid, password FROM employees WHERE email = $1 AND deleted_at IS NULL",
        email
    )
    .fetch_optional(&ctx.pool)
    .await?)
}

pub async fn find_role_by_unid(
    unid: Uuid,
    ctx: &AppState,
) -> Result<Option<EmployeeRole>, AppError> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT role AS "role!: EmployeeRole"
        FROM employees
        WHERE unid = $1
          AND deleted_at IS NULL
        "#,
        unid
    )
    .fetch_optional(&ctx.pool)
    .await?)
}

pub async fn find_current_user_by_unid(
    unid: Uuid,
    ctx: &AppState,
) -> Result<Option<CurrentUser>, AppError> {
    Ok(sqlx::query_as!(
        CurrentUser,
        r#"
        SELECT unid, first_name, last_name, email, role AS "role!: EmployeeRole"
        FROM employees
        WHERE unid = $1
          AND deleted_at IS NULL
        "#,
        unid
    )
    .fetch_optional(&ctx.pool)
    .await?)
}
