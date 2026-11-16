use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::{
    enums::EmployeeRole,
    error::AppError,
    features::auth::{
        jwt::{verify_token, Claims},
        service,
    },
    http::AppState,
};

pub const SESSION_COOKIE: &str = "session";

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub unid: Uuid,
    pub role: EmployeeRole,
}

pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = session_claims(&jar, &state)?;
    let role = service::get_role(claims.sub, &state).await?;

    request.extensions_mut().insert(AuthenticatedUser {
        unid: claims.sub,
        role,
    });

    Ok(next.run(request).await)
}

pub fn session_claims(jar: &CookieJar, state: &AppState) -> Result<Claims, AppError> {
    let token = jar
        .get(SESSION_COOKIE)
        .ok_or(AppError::Unauthorized("no session"))?;

    verify_token(token.value(), state.jwt_secret.expose_secret()).map_err(|e| {
        tracing::warn!(error = %e, "invalid session token");
        AppError::Unauthorized("invalid token")
    })
}
