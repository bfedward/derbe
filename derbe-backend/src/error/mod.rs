use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}, Json
};
use serde_json::json;

pub enum AppError {
    DatabaseQueryError(sqlx::Error),
    EntityNotFound(i32, String),
    RelatedEntity(String, String),
    ConstraintError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::DatabaseQueryError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::EntityNotFound(id, name) => {
                println!("{name} with id {id} not found!");
                (
                StatusCode::NOT_FOUND,
                format!("{name} with id {id} not found!"),
            )},
            Self::RelatedEntity(name, relation) => (
                StatusCode::CONFLICT,
                format!("Cannot delete {name} due to related {relation}!"),
            ),
            Self::ConstraintError(error) => (
                StatusCode::CONFLICT,
                format!("Database constraint error: {error}"),
            ),
        };

        // Return a JSON response
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
