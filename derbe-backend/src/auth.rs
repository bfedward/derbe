use std::str::FromStr;

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;
use uuid::Uuid;

use crate::{configuration::AppEnvironment, enums::EmployeeRole, http::AppState};

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

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_owned();

    let Some(token) = extract_session_cookie(request.headers()) else {
        tracing::warn!(path, "require_auth: no session cookie");
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(json!({"error": "no session"})),
        )
            .into_response();
    };

    let claims = match verify_token(&token, state.jwt_secret.expose_secret()) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::warn!(path, error = %e, "Invalid session token");

            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid token"})),
            )
                .into_response();
        }
    };

    let role = match sqlx::query_scalar!(
        r#"
        SELECT role
        FROM employees
        WHERE unid = $1
          AND deleted_at IS NULL
        "#,
        claims.sub
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(role)) => role,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "employee not found"})),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to retrieve employee role: {e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "internal server error"})),
            )
                .into_response();
        }
    };

    let role = match EmployeeRole::from_str(&role) {
        Ok(role) => role,
        Err(e) => {
            tracing::error!("Invalid employee role in database: {e}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "invalid employee role"})),
            )
                .into_response();
        }
    };

    request.extensions_mut().insert(AuthenticatedUser {
        unid: claims.sub,
        role,
    });

    next.run(request).await
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

fn extract_session_cookie(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("session=") {
            return Some(val.to_string());
        }
    }
    None
}

fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

/// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let row = sqlx::query_as!(
        LoginRow,
        "SELECT unid, password FROM employees WHERE email = $1",
        body.email
    )
    .fetch_optional(&state.pool)
    .await;

    let row = match row {
        Ok(Some(r)) => r,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid credentials"})),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("auth login db: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "db error"})),
            )
                .into_response();
        }
    };

    if !pwhash::unix::verify(&body.password, &row.password) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "invalid credentials"})),
        )
            .into_response();
    }

    let token = match make_token(row.unid, &state) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("jwt encode: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "token error"})),
            )
                .into_response();
        }
    };

    let is_prod = state.app_env == AppEnvironment::Production;
    let cookie = Cookie::build(("session", token))
        .http_only(true)
        .secure(is_prod)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::days(30))
        .build();

    (jar.add(cookie), Json(json!({"ok": true}))).into_response()
}

/// POST /api/auth/logout
pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("session", ""))
        .http_only(true)
        .path("/")
        .max_age(Duration::seconds(0))
        .build();
    (jar.add(cookie), Json(json!({"ok": true}))).into_response()
}
