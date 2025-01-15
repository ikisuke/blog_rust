use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::{Result, UserError},
    models::{Profile, UpdateProfileRequest, User, UserListResponse, UserResponse},
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_username(
        &self,
        username: &str,
        current_user_id: Option<Uuid>,
    ) -> Result<UserResponse>;

    async fn get_profile(&self, user_id: Uuid) -> Result<Profile>;

    async fn update_profile(
        &self,
        user_id: Uuid,
        req: UpdateProfileRequest,
    ) -> Result<Profile>;

    async fn follow_user(
        &self,
        follower_id: Uuid,
        username: &str,
    ) -> Result<()>;

    async fn unfollow_user(
        &self,
        follower_id: Uuid,
        username: &str,
    ) -> Result<()>;

    async fn get_followers(
        &self,
        username: &str,
        page: i32,
        per_page: i32,
        current_user_id: Option<Uuid>,
    ) -> Result<UserListResponse>;

    async fn get_following(
        &self,
        username: &str,
        page: i32,
        per_page: i32,
        current_user_id: Option<Uuid>,
    ) -> Result<UserListResponse>;
}

pub struct MockUserService;

#[async_trait]
impl UserService for MockUserService {
    async fn get_user_by_username(
        &self,
        username: &str,
        _current_user_id: Option<Uuid>,
    ) -> Result<UserResponse> {
        // Mock implementation
        Ok(UserResponse {
            id: Uuid::new_v4(),
            username: username.to_string(),
            display_name: Some("Mock User".to_string()),
            bio: Some("This is a mock user bio".to_string()),
            avatar_url: None,
            website: None,
            location: Some("Mock Location".to_string()),
            social_links: None,
            created_at: Utc::now(),
            followers_count: 0,
            following_count: 0,
            is_following: false,
        })
    }

    async fn get_profile(&self, user_id: Uuid) -> Result<Profile> {
        Ok(Profile {
            user_id,
            website: None,
            location: Some("Mock Location".to_string()),
            social_links: None,
            preferences: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn update_profile(
        &self,
        user_id: Uuid,
        _req: UpdateProfileRequest,
    ) -> Result<Profile> {
        Ok(Profile {
            user_id,
            website: None,
            location: Some("Updated Location".to_string()),
            social_links: None,
            preferences: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn follow_user(
        &self,
        follower_id: Uuid,
        username: &str,
    ) -> Result<()> {
        // Mock implementation
        if username == "self" {
            return Err(UserError::SelfFollow);
        }
        Ok(())
    }

    async fn unfollow_user(
        &self,
        follower_id: Uuid,
        username: &str,
    ) -> Result<()> {
        // Mock implementation
        if username == "self" {
            return Err(UserError::SelfFollow);
        }
        Ok(())
    }

    async fn get_followers(
        &self,
        _username: &str,
        page: i32,
        per_page: i32,
        _current_user_id: Option<Uuid>,
    ) -> Result<UserListResponse> {
        Ok(UserListResponse {
            users: vec![],
            total: 0,
            page,
            per_page,
            total_pages: 0,
        })
    }

    async fn get_following(
        &self,
        _username: &str,
        page: i32,
        per_page: i32,
        _current_user_id: Option<Uuid>,
    ) -> Result<UserListResponse> {
        Ok(UserListResponse {
            users: vec![],
            total: 0,
            page,
            per_page,
            total_pages: 0,
        })
    }
}