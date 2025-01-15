use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub status: CommentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
    Spam,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    pub post_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 1000, message = "Comment must be between 1 and 1000 characters"))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1, max = 1000, message = "Comment must be between 1 and 1000 characters"))]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ModerateCommentRequest {
    pub action: ModerateAction,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModerateAction {
    Approve,
    Reject,
    MarkAsSpam,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author: Option<CommentAuthor>,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub status: CommentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies_count: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentAuthor {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Deserialize)]
pub struct CommentFilters {
    pub status: Option<CommentStatus>,
    pub author_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct ModerationLog {
    pub comment_id: Uuid,
    pub moderator_id: Uuid,
    pub action: ModerateAction,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}