use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserExists,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::UserExists => (StatusCode::CONFLICT, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AuthError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AuthError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AuthError>;