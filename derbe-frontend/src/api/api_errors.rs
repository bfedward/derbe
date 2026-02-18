use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct ApiErrorResponse {
    error: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Expired or missing auth token")]
    NotAuthenticated,
    #[error("Unknown Network error")]
    Unknown,
}

pub fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}
