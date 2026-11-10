use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    error::AppError,
    features::divisions::dto::{request::DivisionCreate, response::DivisionResponse},
    features::divisions::service,
    http::AppState,
};

pub async fn list_divisions(
    State(state): State<AppState>,
) -> Result<Json<Vec<DivisionResponse>>, AppError> {
    let divisions = service::list(&state).await?;
    Ok(Json(
        divisions.into_iter().map(DivisionResponse::from).collect(),
    ))
}

pub async fn get_division(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<DivisionResponse>, AppError> {
    let division = service::get_by_id(id, &state).await?;
    Ok(Json(division.into()))
}

pub async fn create_division(
    State(state): State<AppState>,
    Json(input): Json<DivisionCreate>,
) -> Result<(StatusCode, Json<DivisionResponse>), AppError> {
    input
        .validate()
        .map_err(AppError::RequestValidationFailed)?;
    let division = service::create(input, &state).await?;
    Ok((StatusCode::CREATED, Json(division.into())))
}
