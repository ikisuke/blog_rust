use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::{CommentError, Result},
    models::{
        Comment, CommentAuthor, CommentFilters, CommentListResponse, CommentResponse,
        CommentStatus, CreateCommentRequest, ModerateAction, ModerateCommentRequest,
        UpdateCommentRequest,
    },
};

const MAX_NESTING_LEVEL: i32 = 3;

#[async_trait]
pub trait CommentService: Send + Sync {
    async fn list_comments(
        &self,
        post_id: Uuid,
        page: i32,
        per_page: i32,
        filters: Option<CommentFilters>,
    ) -> Result<CommentListResponse>;

    async fn get_comment(&self, id: Uuid) -> Result<CommentResponse>;

    async fn create_comment(
        &self,
        author_id: Option<Uuid>,
        req: CreateCommentRequest,
    ) -> Result<CommentResponse>;

    async fn update_comment(
        &self,
        id: Uuid,
        author_id: Option<Uuid>,
        req: UpdateCommentRequest,
    ) -> Result<CommentResponse>;

    async fn delete_comment(
        &self,
        id: Uuid,
        author_id: Option<Uuid>,
    ) -> Result<()>;

    async fn moderate_comment(
        &self,
        id: Uuid,
        moderator_id: Uuid,
        req: ModerateCommentRequest,
    ) -> Result<CommentResponse>;

    async fn list_replies(
        &self,
        parent_id: Uuid,
        page: i32,
        per_page: i32,
    ) -> Result<CommentListResponse>;
}

pub struct MockCommentService;

#[async_trait]
impl CommentService for MockCommentService {
    async fn list_comments(
        &self,
        post_id: Uuid,
        page: i32,
        per_page: i32,
        _filters: Option<CommentFilters>,
    ) -> Result<CommentListResponse> {
        Ok(CommentListResponse {
            comments: vec![],
            total: 0,
            page,
            per_page,
            total_pages: 0,
        })
    }

    async fn get_comment(&self, id: Uuid) -> Result<CommentResponse> {
        Ok(CommentResponse {
            id,
            post_id: Uuid::new_v4(),
            author: Some(CommentAuthor {
                id: Uuid::new_v4(),
                username: "mockuser".to_string(),
                display_name: Some("Mock User".to_string()),
                avatar_url: None,
            }),
            parent_id: None,
            content: "Mock comment".to_string(),
            status: CommentStatus::Approved,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            replies_count: 0,
        })
    }

    async fn create_comment(
        &self,
        author_id: Option<Uuid>,
        req: CreateCommentRequest,
    ) -> Result<CommentResponse> {
        if let Some(parent_id) = req.parent_id {
            // Check nesting level
            let mut current_id = Some(parent_id);
            let mut nesting_level = 0;

            while let Some(id) = current_id {
                nesting_level += 1;
                if nesting_level > MAX_NESTING_LEVEL {
                    return Err(CommentError::MaxNestingLevel);
                }
                // In a real implementation, we would fetch the parent comment
                // and update current_id with its parent_id
                current_id = None;
            }
        }

        Ok(CommentResponse {
            id: Uuid::new_v4(),
            post_id: req.post_id,
            author: author_id.map(|id| CommentAuthor {
                id,
                username: "mockuser".to_string(),
                display_name: Some("Mock User".to_string()),
                avatar_url: None,
            }),
            parent_id: req.parent_id,
            content: req.content,
            status: CommentStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            replies_count: 0,
        })
    }

    async fn update_comment(
        &self,
        id: Uuid,
        _author_id: Option<Uuid>,
        req: UpdateCommentRequest,
    ) -> Result<CommentResponse> {
        Ok(CommentResponse {
            id,
            post_id: Uuid::new_v4(),
            author: Some(CommentAuthor {
                id: Uuid::new_v4(),
                username: "mockuser".to_string(),
                display_name: Some("Mock User".to_string()),
                avatar_url: None,
            }),
            parent_id: None,
            content: req.content,
            status: CommentStatus::Approved,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            replies_count: 0,
        })
    }

    async fn delete_comment(
        &self,
        _id: Uuid,
        _author_id: Option<Uuid>,
    ) -> Result<()> {
        Ok(())
    }

    async fn moderate_comment(
        &self,
        id: Uuid,
        _moderator_id: Uuid,
        req: ModerateCommentRequest,
    ) -> Result<CommentResponse> {
        let status = match req.action {
            ModerateAction::Approve => CommentStatus::Approved,
            ModerateAction::Reject => CommentStatus::Rejected,
            ModerateAction::MarkAsSpam => CommentStatus::Spam,
        };

        Ok(CommentResponse {
            id,
            post_id: Uuid::new_v4(),
            author: Some(CommentAuthor {
                id: Uuid::new_v4(),
                username: "mockuser".to_string(),
                display_name: Some("Mock User".to_string()),
                avatar_url: None,
            }),
            parent_id: None,
            content: "Mock comment".to_string(),
            status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            replies_count: 0,
        })
    }

    async fn list_replies(
        &self,
        _parent_id: Uuid,
        page: i32,
        per_page: i32,
    ) -> Result<CommentListResponse> {
        Ok(CommentListResponse {
            comments: vec![],
            total: 0,
            page,
            per_page,
            total_pages: 0,
        })
    }
}