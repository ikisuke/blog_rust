use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::Result,
    models::PaginationParams,
    services::{UserService, MockUserService},
};

pub async fn follow_user(
    Path(username): Path<String>,
    State(service): State<MockUserService>,
) -> Result<StatusCode> {
    // TODO: Get follower_id from authenticated user
    let follower_id = Uuid::new_v4();
    service.follow_user(follower_id, &username).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn unfollow_user(
    Path(username): Path<String>,
    State(service): State<MockUserService>,
) -> Result<StatusCode> {
    // TODO: Get follower_id from authenticated user
    let follower_id = Uuid::new_v4();
    service.unfollow_user(follower_id, &username).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_followers(
    Path(username): Path<String>,
    Query(params): Query<PaginationParams>,
    State(service): State<MockUserService>,
) -> Result<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    // TODO: Get current_user_id from authenticated user
    let current_user_id = None;

    let response = service
        .get_followers(&username, page, per_page, current_user_id)
        .await?;

    Ok(Json(serde_json::json!({
        "users": response.users,
        "pagination": {
            "total": response.total,
            "page": response.page,
            "per_page": response.per_page,
            "total_pages": response.total_pages
        }
    })))
}

pub async fn get_following(
    Path(username): Path<String>,
    Query(params): Query<PaginationParams>,
    State(service): State<MockUserService>,
) -> Result<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    // TODO: Get current_user_id from authenticated user
    let current_user_id = None;

    let response = service
        .get_following(&username, page, per_page, current_user_id)
        .await?;

    Ok(Json(serde_json::json!({
        "users": response.users,
        "pagination": {
            "total": response.total,
            "page": response.page,
            "per_page": response.per_page,
            "total_pages": response.total_pages
        }
    })))
}