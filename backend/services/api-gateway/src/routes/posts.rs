use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    error::Result,
    middleware::AuthUser,
    services::{mock::MockPostService, PostService},
    types::{CreatePostRequest, PaginationParams, UpdatePostRequest},
};

pub fn router() -> Router {
    let post_service = MockPostService;

    Router::new()
        .route("/posts", get(get_posts).post(create_post))
        .route(
            "/posts/:id",
            get(get_post_by_slug)
                .put(update_post)
                .delete(delete_post),
        )
        .with_state(post_service)
}

async fn get_posts(
    Query(params): Query<PaginationParams>,
    State(service): State<MockPostService>,
) -> Result<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let (posts, total) = service.get_posts(page, per_page, None, None).await?;

    Ok(Json(serde_json::json!({
        "posts": posts,
        "pagination": {
            "total": total,
            "page": page,
            "per_page": per_page,
            "total_pages": (total as f64 / per_page as f64).ceil() as u32
        }
    })))
}

async fn get_post_by_slug(
    Path(slug): Path<String>,
    State(service): State<MockPostService>,
) -> Result<Json<serde_json::Value>> {
    let post = service.get_post_by_slug(&slug).await?;
    Ok(Json(serde_json::json!({ "post": post })))
}

async fn create_post(
    auth: AuthUser,
    State(service): State<MockPostService>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>> {
    let post = service.create_post(auth.id, req).await?;
    Ok(Json(serde_json::json!({ "post": post })))
}

async fn update_post(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(service): State<MockPostService>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<serde_json::Value>> {
    let post = service.update_post(id, auth.id, req).await?;
    Ok(Json(serde_json::json!({ "post": post })))
}

async fn delete_post(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(service): State<MockPostService>,
) -> Result<Json<serde_json::Value>> {
    service.delete_post(id, auth.id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}