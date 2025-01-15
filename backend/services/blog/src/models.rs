use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub category_ids: Vec<Uuid>,
    pub tag_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub status: Option<PostStatus>,
    pub category_ids: Option<Vec<Uuid>>,
    pub tag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub status: PostStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, page: u32, per_page: u32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostFilters {
    pub status: Option<PostStatus>,
    pub author_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
}