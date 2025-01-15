use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogError {
    #[error("Post not found")]
    PostNotFound,

    #[error("Category not found")]
    CategoryNotFound,

    #[error("Tag not found")]
    TagNotFound,

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

    #[error("Slug already exists")]
    SlugExists,
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BlogError::PostNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            BlogError::CategoryNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            BlogError::TagNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            BlogError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            BlogError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            BlogError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            BlogError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            BlogError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            BlogError::SlugExists => (
                StatusCode::CONFLICT,
                "A post with this slug already exists".to_string(),
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

pub type Result<T> = std::result::Result<T, BlogError>;