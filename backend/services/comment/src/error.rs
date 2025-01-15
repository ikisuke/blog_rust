use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommentError {
    #[error("Comment not found")]
    CommentNotFound,

    #[error("Post not found")]
    PostNotFound,

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

    #[error("Comment is already moderated")]
    AlreadyModerated,

    #[error("Invalid moderation action")]
    InvalidModeration,

    #[error("Maximum nesting level reached")]
    MaxNestingLevel,

    #[error("Comment is deleted")]
    CommentDeleted,

    #[error("Parent comment not found")]
    ParentNotFound,

    #[error("Parent comment is not approved")]
    ParentNotApproved,
}

impl IntoResponse for CommentError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CommentError::CommentNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            CommentError::PostNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            CommentError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            CommentError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            CommentError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            CommentError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            CommentError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            CommentError::AlreadyModerated => (
                StatusCode::CONFLICT,
                "This comment has already been moderated".to_string(),
            ),
            CommentError::InvalidModeration => (
                StatusCode::BAD_REQUEST,
                "Invalid moderation action".to_string(),
            ),
            CommentError::MaxNestingLevel => (
                StatusCode::BAD_REQUEST,
                "Maximum comment nesting level reached".to_string(),
            ),
            CommentError::CommentDeleted => (
                StatusCode::GONE,
                "This comment has been deleted".to_string(),
            ),
            CommentError::ParentNotFound => (
                StatusCode::BAD_REQUEST,
                "Parent comment not found".to_string(),
            ),
            CommentError::ParentNotApproved => (
                StatusCode::BAD_REQUEST,
                "Cannot reply to unapproved comment".to_string(),
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

pub type Result<T> = std::result::Result<T, CommentError>;