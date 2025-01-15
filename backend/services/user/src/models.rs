use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: Uuid,
    pub website: Option<String>,
    pub location: Option<String>,
    pub social_links: Option<SocialLinks>,
    pub preferences: Option<UserPreferences>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialLinks {
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub email_notifications: bool,
    pub newsletter_subscription: bool,
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 100, message = "Display name must be between 1 and 100 characters"))]
    pub display_name: Option<String>,

    #[validate(length(max = 500, message = "Bio must not exceed 500 characters"))]
    pub bio: Option<String>,

    #[validate(url(message = "Website must be a valid URL"))]
    pub website: Option<String>,

    pub location: Option<String>,
    pub avatar_url: Option<String>,
    pub social_links: Option<SocialLinks>,
    pub preferences: Option<UserPreferences>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub social_links: Option<SocialLinks>,
    pub created_at: DateTime<Utc>,
    pub followers_count: i64,
    pub following_count: i64,
    pub is_following: bool,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
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