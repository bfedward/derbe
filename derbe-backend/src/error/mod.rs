use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    DatabaseQueryError(sqlx::Error),
    EntityNotFound(i32, String),
    RelatedEntity(String),
    ConstraintError(String),
    InvalidStructArgs(String),
    ValidationFailed(String),
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
                )
            }
            Self::RelatedEntity(name) => (
                StatusCode::CONFLICT,
                format!("Cannot delete {name} due to related entity!"),
            ),
            Self::ConstraintError(error) => (
                StatusCode::CONFLICT,
                format!("Database constraint error: {error}"),
            ),
            Self::InvalidStructArgs(error) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid struct args: {error}"),
            ),
            Self::ValidationFailed(message) => (StatusCode::BAD_REQUEST, message),
        };

        // Return a JSON response
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
