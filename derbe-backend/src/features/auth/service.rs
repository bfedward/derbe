use axum_extra::extract::cookie::{Cookie, SameSite};
use secrecy::ExposeSecret;
use time::Duration;
use uuid::Uuid;

use crate::{
    configuration::AppEnvironment,
    error::AppError,
    features::auth::{
        dto::LoginRequest, jwt::make_token, middleware::SESSION_COOKIE, model::CurrentUser,
        repository,
    },
    features::employees::model::EmployeeRole,
    http::AppState,
};

/// Checks the credentials and returns a session token.
pub async fn login(input: LoginRequest, ctx: &AppState) -> Result<String, AppError> {
    let credentials = repository::find_login_by_email(&input.email.trim().to_lowercase(), ctx)
        .await?
        .ok_or(AppError::Unauthorized("invalid credentials"))?;

    if !pwhash::unix::verify(&input.password, &credentials.password) {
        return Err(AppError::Unauthorized("invalid credentials"));
    }

    make_token(credentials.unid, ctx.jwt_secret.expose_secret()).map_err(|e| {
        tracing::error!("jwt encode: {e}");
        AppError::TokenFailure
    })
}

pub async fn get_role(unid: Uuid, ctx: &AppState) -> Result<EmployeeRole, AppError> {
    repository::find_role_by_unid(unid, ctx)
        .await?
        .ok_or(AppError::Unauthorized("employee not found"))
}

pub async fn get_current_user(unid: Uuid, ctx: &AppState) -> Result<CurrentUser, AppError> {
    repository::find_current_user_by_unid(unid, ctx)
        .await?
        .ok_or(AppError::Unauthorized("user not found"))
}

pub fn session_cookie(token: String, ctx: &AppState) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, token))
        .http_only(true)
        .secure(ctx.app_env == AppEnvironment::Production)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::days(30))
        .build()
}

pub fn expired_session_cookie() -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, ""))
        .http_only(true)
        .path("/")
        .max_age(Duration::seconds(0))
        .build()
}
