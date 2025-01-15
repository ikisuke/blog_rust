use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::Result,
    types::{
        Comment, CommentStatus, CreateCommentRequest, CreatePostRequest, Post, PostStatus,
        UpdateCommentRequest, UpdatePostRequest, User,
    },
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn validate_token(&self, token: &str) -> Result<User>;
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn get_posts(
        &self,
        page: u32,
        per_page: u32,
        status: Option<PostStatus>,
        author_id: Option<Uuid>,
    ) -> Result<(Vec<Post>, u64)>;

    async fn get_post_by_slug(&self, slug: &str) -> Result<Post>;

    async fn create_post(&self, author_id: Uuid, req: CreatePostRequest) -> Result<Post>;

    async fn update_post(&self, id: Uuid, author_id: Uuid, req: UpdatePostRequest) -> Result<Post>;

    async fn delete_post(&self, id: Uuid, author_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User>;

    async fn get_user_by_username(&self, username: &str) -> Result<User>;

    async fn update_user(&self, id: Uuid, user: User) -> Result<User>;
}

#[async_trait]
pub trait CommentService: Send + Sync {
    async fn get_comments(
        &self,
        post_id: Uuid,
        page: u32,
        per_page: u32,
        status: Option<CommentStatus>,
    ) -> Result<(Vec<Comment>, u64)>;

    async fn create_comment(
        &self,
        post_id: Uuid,
        author_id: Option<Uuid>,
        req: CreateCommentRequest,
    ) -> Result<Comment>;

    async fn update_comment(
        &self,
        id: Uuid,
        author_id: Option<Uuid>,
        req: UpdateCommentRequest,
    ) -> Result<Comment>;

    async fn delete_comment(&self, id: Uuid, author_id: Option<Uuid>) -> Result<()>;
}

// Mock implementations for development
pub mod mock {
    use super::*;
    use chrono::Utc;

    pub struct MockAuthService;
    pub struct MockPostService;
    pub struct MockUserService;
    pub struct MockCommentService;

    #[async_trait]
    impl AuthService for MockAuthService {
        async fn validate_token(&self, _token: &str) -> Result<User> {
            Ok(User {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                display_name: Some("Test User".to_string()),
                bio: None,
                avatar_url: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }
    }

    #[async_trait]
    impl PostService for MockPostService {
        async fn get_posts(
            &self,
            _page: u32,
            _per_page: u32,
            _status: Option<PostStatus>,
            _author_id: Option<Uuid>,
        ) -> Result<(Vec<Post>, u64)> {
            Ok((vec![], 0))
        }

        async fn get_post_by_slug(&self, _slug: &str) -> Result<Post> {
            Ok(Post {
                id: Uuid::new_v4(),
                author_id: Uuid::new_v4(),
                title: "Test Post".to_string(),
                slug: "test-post".to_string(),
                content: "Test content".to_string(),
                excerpt: None,
                status: PostStatus::Published,
                published_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn create_post(&self, _author_id: Uuid, _req: CreatePostRequest) -> Result<Post> {
            Ok(Post {
                id: Uuid::new_v4(),
                author_id: Uuid::new_v4(),
                title: "Test Post".to_string(),
                slug: "test-post".to_string(),
                content: "Test content".to_string(),
                excerpt: None,
                status: PostStatus::Draft,
                published_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_post(
            &self,
            _id: Uuid,
            _author_id: Uuid,
            _req: UpdatePostRequest,
        ) -> Result<Post> {
            Ok(Post {
                id: Uuid::new_v4(),
                author_id: Uuid::new_v4(),
                title: "Updated Post".to_string(),
                slug: "updated-post".to_string(),
                content: "Updated content".to_string(),
                excerpt: None,
                status: PostStatus::Published,
                published_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn delete_post(&self, _id: Uuid, _author_id: Uuid) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl UserService for MockUserService {
        async fn get_user_by_id(&self, _id: Uuid) -> Result<User> {
            Ok(User {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                display_name: Some("Test User".to_string()),
                bio: None,
                avatar_url: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn get_user_by_username(&self, _username: &str) -> Result<User> {
            Ok(User {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                display_name: Some("Test User".to_string()),
                bio: None,
                avatar_url: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_user(&self, _id: Uuid, user: User) -> Result<User> {
            Ok(user)
        }
    }

    #[async_trait]
    impl CommentService for MockCommentService {
        async fn get_comments(
            &self,
            _post_id: Uuid,
            _page: u32,
            _per_page: u32,
            _status: Option<CommentStatus>,
        ) -> Result<(Vec<Comment>, u64)> {
            Ok((vec![], 0))
        }

        async fn create_comment(
            &self,
            _post_id: Uuid,
            _author_id: Option<Uuid>,
            _req: CreateCommentRequest,
        ) -> Result<Comment> {
            Ok(Comment {
                id: Uuid::new_v4(),
                post_id: Uuid::new_v4(),
                author_id: Some(Uuid::new_v4()),
                parent_id: None,
                content: "Test comment".to_string(),
                status: CommentStatus::Pending,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_comment(
            &self,
            _id: Uuid,
            _author_id: Option<Uuid>,
            _req: UpdateCommentRequest,
        ) -> Result<Comment> {
            Ok(Comment {
                id: Uuid::new_v4(),
                post_id: Uuid::new_v4(),
                author_id: Some(Uuid::new_v4()),
                parent_id: None,
                content: "Updated comment".to_string(),
                status: CommentStatus::Approved,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn delete_comment(&self, _id: Uuid, _author_id: Option<Uuid>) -> Result<()> {
            Ok(())
        }
    }
}