use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::Duration;
use uuid::Uuid;

use crate::{configuration::AppEnvironment, enums::EmployeeRole, error::AppError, http::AppState};

const SESSION_COOKIE: &str = "session";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid, // user unid
    pub exp: usize,      // unix timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

struct LoginRow {
    unid: Uuid,
    password: String,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub unid: Uuid,
    pub role: EmployeeRole,
}

struct MeRow {
    unid: Uuid,
    email: String,
    role: EmployeeRole,
}

pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = session_claims(&jar, &state)?;

    let role = sqlx::query_scalar!(
        r#"
        SELECT role AS "role!: EmployeeRole"
        FROM employees
        WHERE unid = $1
          AND deleted_at IS NULL
        "#,
        claims.sub
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("employee not found"))?;

    request.extensions_mut().insert(AuthenticatedUser {
        unid: claims.sub,
        role,
    });

    Ok(next.run(request).await)
}

fn session_claims(jar: &CookieJar, state: &AppState) -> Result<Claims, AppError> {
    let token = jar
        .get(SESSION_COOKIE)
        .ok_or(AppError::Unauthorized("no session"))?;

    verify_token(token.value(), state.jwt_secret.expose_secret()).map_err(|e| {
        tracing::warn!(error = %e, "invalid session token");
        AppError::Unauthorized("invalid token")
    })
}

fn make_token(user_unid: Uuid, state: &AppState) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;
    encode(
        &Header::default(),
        &Claims {
            sub: user_unid,
            exp,
        },
        &EncodingKey::from_secret(state.jwt_secret.expose_secret().as_bytes()),
    )
}

fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<Value>), AppError> {
    let row = sqlx::query_as!(
        LoginRow,
        "SELECT unid, password FROM employees WHERE email = $1 AND deleted_at IS NULL",
        body.email
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("invalid credentials"))?;

    if !pwhash::unix::verify(&body.password, &row.password) {
        return Err(AppError::Unauthorized("invalid credentials"));
    }

    let token = make_token(row.unid, &state).map_err(|e| {
        tracing::error!("jwt encode: {e}");
        AppError::TokenFailure
    })?;

    let is_prod = state.app_env == AppEnvironment::Production;
    let cookie = Cookie::build((SESSION_COOKIE, token))
        .http_only(true)
        .secure(is_prod)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::days(30))
        .build();

    Ok((jar.add(cookie), Json(json!({"ok": true}))))
}

pub async fn me(State(state): State<AppState>, jar: CookieJar) -> Result<Json<Value>, AppError> {
    let claims = session_claims(&jar, &state)?;

    let row = sqlx::query_as!(
        MeRow,
        r#"
        SELECT unid, email, role AS "role!: EmployeeRole"
        FROM employees
        WHERE unid = $1
          AND deleted_at IS NULL
        "#,
        claims.sub
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("user not found"))?;

    Ok(Json(json!({
        "unid": row.unid,
        "email": row.email,
        "role": row.role
    })))
}

pub async fn logout(jar: CookieJar) -> (CookieJar, Json<Value>) {
    let cookie = Cookie::build((SESSION_COOKIE, ""))
        .http_only(true)
        .path("/")
        .max_age(Duration::seconds(0))
        .build();
    (jar.add(cookie), Json(json!({"ok": true})))
}
