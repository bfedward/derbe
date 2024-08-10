use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use derbe_shared::dto::response::division::DivisionResponse;

use crate::{error::AppError, http::AppContext, service::division_service};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/divisions", get(get_divisions))
        .route("/divisions/:id", get(get_division))
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
