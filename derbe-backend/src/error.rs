use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    DatabaseQueryError(#[from] sqlx::Error),

    #[error("{1} with id {0} not found!")]
    EntityNotFound(i32, String),

    #[error("{1} with id {0} not found!")]
    EntityUnidNotFound(Uuid, String),

    #[error("Cannot delete {0} due to related entity!")]
    RelatedEntity(String),

    #[error("Database constraint error: {0}")]
    ConstraintError(String),

    #[error("Invalid struct args: {0}")]
    InvalidStructArgs(String),

    #[error("{0}")]
    ValidationFailed(String),

    #[error("{0}")]
    RequestValidationFailed(ValidationErrors),

    #[error("{0} with this name already exists!")]
    DuplicateNamedEntity(String),

    #[error("{0} is not a valid UUID")]
    InvalidUuid(String),

    #[error("Failed to hash")]
    HashFailure,

    #[error("Failed to create session token")]
    TokenFailure,

    #[error("{0}")]
    Unauthorized(&'static str),

    #[error("Forbidden")]
    Forbidden,
}

impl AppError {
    /// Turns a unique violation into `DuplicateNamedEntity`, any other database error stays a 500.
    pub fn duplicate_name_or_db(error: sqlx::Error, entity: &str) -> Self {
        match &error {
            sqlx::Error::Database(db_error) if db_error.is_unique_violation() => {
                Self::DuplicateNamedEntity(entity.to_owned())
            }
            _ => Self::DatabaseQueryError(error),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let Self::DatabaseQueryError(error) = &self {
            tracing::error!(error = %error, "database query failed");
        }

        let status = match &self {
            Self::DatabaseQueryError(_) | Self::HashFailure | Self::TokenFailure => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::EntityUnidNotFound(_, _) => StatusCode::NOT_FOUND,
            Self::EntityNotFound(_, _) => StatusCode::NOT_FOUND,
            Self::RelatedEntity(_) | Self::ConstraintError(_) | Self::DuplicateNamedEntity(_) => {
                StatusCode::CONFLICT
            }
            Self::InvalidStructArgs(_)
            | Self::ValidationFailed(_)
            | Self::RequestValidationFailed(_)
            | Self::InvalidUuid(_) => StatusCode::BAD_REQUEST,
            Self::Forbidden => StatusCode::FORBIDDEN,
        };

        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_is_a_401_with_its_message() {
        let response = AppError::Unauthorized("no session").into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn token_failure_is_a_500() {
        let response = AppError::TokenFailure.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
