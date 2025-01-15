use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,

    #[error("Profile not found")]
    ProfileNotFound,

    #[error("Permission denied")]
    Forbidden,

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Username already exists")]
    UsernameExists,

    #[error("Cannot follow yourself")]
    SelfFollow,

    #[error("Already following this user")]
    AlreadyFollowing,

    #[error("Not following this user")]
    NotFollowing,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            UserError::ProfileNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            UserError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            UserError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            UserError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            UserError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            UserError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            UserError::UsernameExists => (
                StatusCode::CONFLICT,
                "Username is already taken".to_string(),
            ),
            UserError::SelfFollow => (
                StatusCode::BAD_REQUEST,
                "You cannot follow yourself".to_string(),
            ),
            UserError::AlreadyFollowing => (
                StatusCode::CONFLICT,
                "You are already following this user".to_string(),
            ),
            UserError::NotFollowing => (
                StatusCode::BAD_REQUEST,
                "You are not following this user".to_string(),
            ),
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

pub type Result<T> = std::result::Result<T, UserError>;