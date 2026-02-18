use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use derbe_shared::dto::{request::division::DivisionCreate, response::division::DivisionResponse};

use crate::{error::AppError, http::AppContext, service::division_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/divisions", get(get_divisions))
        .route("/divisions/{id}", get(get_division))
        .route("/divisions", post(create_division))
}

async fn get_divisions(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<DivisionResponse>>, AppError> {
    match division_service::get_entities(&ctx).await {
        Ok(divisions) => {
            let resps = divisions.into_iter().map(DivisionResponse::from).collect();
            Ok(Json(resps))
        }
        Err(e) => Err(e),
    }
}

async fn get_division(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<DivisionResponse>, AppError> {
    match division_service::get_entity_by_id(id, &ctx).await {
        Ok(division) => {
            let resp = division.into();
            Ok(Json(resp))
        }
        Err(e) => Err(e),
    }
}

async fn create_division(
    State(ctx): State<AppContext>,
    Json(division): Json<DivisionCreate>,
) -> Result<Response, AppError> {
    match division_service::create_entity(division, &ctx).await {
        Ok(division) => {
            let division: DivisionResponse = division.into();
            let response = (StatusCode::CREATED, Json(division)).into_response();
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
