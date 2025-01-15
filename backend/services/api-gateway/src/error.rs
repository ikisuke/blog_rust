use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Permission denied")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            ApiError::ServiceError(msg) => {
                tracing::error!("Service error: {}", msg);
                (StatusCode::BAD_GATEWAY, msg)
            }
            ApiError::RateLimitExceeded => {
                (StatusCode::TOO_MANY_REQUESTS, self.to_string())
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

pub type Result<T> = std::result::Result<T, ApiError>;