use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use derbe_core::access_control::authorize;
use derbe_core::error::AppError;
use derbe_core::session::AuthenticatedUser;
use derbe_core::state::AppState;
use validator::Validate;

use crate::{
    permission::DivisionPermission,
    {
        dto::{request::DivisionCreate, response::DivisionResponse},
        service,
    },
};

pub async fn list_divisions(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<DivisionResponse>>, AppError> {
    authorize(&user, DivisionPermission::View)?;
    let divisions = service::list(&state).await?;
    Ok(Json(divisions))
}

pub async fn get_division(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<DivisionResponse>, AppError> {
    authorize(&user, DivisionPermission::View)?;
    let division = service::get_by_id(id, &state).await?;
    Ok(Json(division))
}

pub async fn create_division(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(input): Json<DivisionCreate>,
) -> Result<(StatusCode, Json<DivisionResponse>), AppError> {
    authorize(&user, DivisionPermission::Create)?;
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let division = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(division)))
}
