use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::AppError,
    http::AppContext,
    model::division::Division,
    service::{division_service::DivisionService, DerbeService},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/divisions", get(get_divisions))
        .route("/divisions/:id", get(get_division))
}

async fn get_divisions(State(ctx): State<AppContext>) -> Result<Json<Vec<Division>>, AppError> {
    match DivisionService::get_entities(&ctx).await {
        Ok(leave_types) => Ok(Json(leave_types)),
        Err(e) => Err(e),
    }
}

async fn get_division(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<Division>, AppError> {
    match DivisionService::get_entity_by_id(id, &ctx).await {
        Ok(leave_type) => Ok(Json(leave_type)),
        Err(e) => Err(e),
    }
}
