use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use serde_json::{json, Value};

use crate::{
    error::AppError,
    features::auth::{dto::LoginRequest, middleware::session_claims, model::CurrentUser, service},
    http::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(input): Json<LoginRequest>,
) -> Result<(CookieJar, Json<Value>), AppError> {
    let token = service::login(input, &state).await?;
    let jar = jar.add(service::session_cookie(token, &state));
    Ok((jar, Json(json!({"ok": true}))))
}

pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<CurrentUser>, AppError> {
    let claims = session_claims(&jar, &state)?;
    let user = service::get_current_user(claims.sub, &state).await?;
    Ok(Json(user))
}

pub async fn logout(jar: CookieJar) -> (CookieJar, Json<Value>) {
    (
        jar.add(service::expired_session_cookie()),
        Json(json!({"ok": true})),
    )
}
