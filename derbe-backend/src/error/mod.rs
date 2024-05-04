use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    DatabaseQueryError(sqlx::Error),
    EntityNotFound(i32, String),
    RelatedEntity(String, String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::DatabaseQueryError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::EntityNotFound(id, name) => (
                StatusCode::NOT_FOUND,
                format!("{} with id {} not found!", name, id),
            ),
            Self::RelatedEntity(name, relation) => (
                StatusCode::CONFLICT,
                format!("Cannot delete {} due to related {}!", name, relation),
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("Unknown server error!"))
        };

        (status, message).into_response()
    }
}
