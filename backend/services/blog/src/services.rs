use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::{BlogError, Result},
    models::{
        Category, CreatePostRequest, PaginatedResponse, Post, PostFilters, PostResponse,
        PostStatus, Tag, UpdatePostRequest,
    },
};

#[async_trait]
pub trait BlogService: Send + Sync {
    async fn list_posts(
        &self,
        page: u32,
        per_page: u32,
        filters: Option<PostFilters>,
    ) -> Result<PaginatedResponse<PostResponse>>;

    async fn get_post(&self, id: Uuid) -> Result<PostResponse>;
    async fn get_post_by_slug(&self, slug: &str) -> Result<PostResponse>;
    async fn create_post(&self, author_id: Uuid, req: CreatePostRequest) -> Result<PostResponse>;
    async fn update_post(
        &self,
        id: Uuid,
        author_id: Uuid,
        req: UpdatePostRequest,
    ) -> Result<PostResponse>;
    async fn delete_post(&self, id: Uuid, author_id: Uuid) -> Result<()>;

    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn list_tags(&self) -> Result<Vec<Tag>>;
}

pub struct MockBlogService;

#[async_trait]
impl BlogService for MockBlogService {
    async fn list_posts(
        &self,
        page: u32,
        per_page: u32,
        _filters: Option<PostFilters>,
    ) -> Result<PaginatedResponse<PostResponse>> {
        // Mock implementation
        let post = PostResponse {
            id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            title: "Sample Post".to_string(),
            slug: "sample-post".to_string(),
            content: "This is a sample post content.".to_string(),
            excerpt: Some("Sample excerpt".to_string()),
            status: PostStatus::Published,
            published_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            categories: vec![],
            tags: vec![],
        };

        Ok(PaginatedResponse::new(vec![post], 1, page, per_page))
    }

    async fn get_post(&self, _id: Uuid) -> Result<PostResponse> {
        Ok(PostResponse {
            id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            title: "Sample Post".to_string(),
            slug: "sample-post".to_string(),
            content: "This is a sample post content.".to_string(),
            excerpt: Some("Sample excerpt".to_string()),
            status: PostStatus::Published,
            published_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            categories: vec![],
            tags: vec![],
        })
    }

    async fn get_post_by_slug(&self, _slug: &str) -> Result<PostResponse> {
        Ok(PostResponse {
            id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            title: "Sample Post".to_string(),
            slug: "sample-post".to_string(),
            content: "This is a sample post content.".to_string(),
            excerpt: Some("Sample excerpt".to_string()),
            status: PostStatus::Published,
            published_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            categories: vec![],
            tags: vec![],
        })
    }

    async fn create_post(&self, author_id: Uuid, req: CreatePostRequest) -> Result<PostResponse> {
        let slug = slug::slugify(&req.title);
        
        Ok(PostResponse {
            id: Uuid::new_v4(),
            author_id,
            title: req.title,
            slug,
            content: req.content,
            excerpt: req.excerpt,
            status: req.status,
            published_at: if req.status == PostStatus::Published {
                Some(Utc::now())
            } else {
                None
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            categories: vec![],
            tags: vec![],
        })
    }

    async fn update_post(
        &self,
        id: Uuid,
        author_id: Uuid,
        req: UpdatePostRequest,
    ) -> Result<PostResponse> {
        let slug = req
            .title
            .as_ref()
            .map(|title| slug::slugify(title))
            .unwrap_or_else(|| "sample-post".to_string());

        Ok(PostResponse {
            id,
            author_id,
            title: req.title.unwrap_or_else(|| "Sample Post".to_string()),
            slug,
            content: req.content.unwrap_or_else(|| "Sample content".to_string()),
            excerpt: req.excerpt,
            status: req.status.unwrap_or(PostStatus::Draft),
            published_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            categories: vec![],
            tags: vec![],
        })
    }

    async fn delete_post(&self, _id: Uuid, _author_id: Uuid) -> Result<()> {
        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        Ok(vec![Category {
            id: Uuid::new_v4(),
            name: "Sample Category".to_string(),
            slug: "sample-category".to_string(),
            description: Some("Sample description".to_string()),
            parent_id: None,
            created_at: Utc::now(),
        }])
    }

    async fn list_tags(&self) -> Result<Vec<Tag>> {
        Ok(vec![Tag {
            id: Uuid::new_v4(),
            name: "Sample Tag".to_string(),
            slug: "sample-tag".to_string(),
            created_at: Utc::now(),
        }])
    }
}