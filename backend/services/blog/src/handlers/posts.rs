use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::Result,
    models::{CreatePostRequest, PaginationParams, PostFilters, UpdatePostRequest},
    services::{BlogService, MockBlogService},
};

pub async fn list_posts(
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<PostFilters>,
    State(service): State<MockBlogService>,
) -> Result<Json<serde_json::Value>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let response = service.list_posts(page, per_page, Some(filters)).await?;

    Ok(Json(serde_json::json!({
        "posts": response.items,
        "pagination": {
            "total": response.total,
            "page": response.page,
            "per_page": response.per_page,
            "total_pages": response.total_pages
        }
    })))
}

pub async fn get_post(
    Path(id): Path<Uuid>,
    State(service): State<MockBlogService>,
) -> Result<Json<serde_json::Value>> {
    let post = service.get_post(id).await?;
    Ok(Json(serde_json::json!({ "post": post })))
}

pub async fn create_post(
    State(service): State<MockBlogService>,
    Json(req): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    // TODO: Get author_id from authenticated user
    let author_id = Uuid::new_v4();
    let post = service.create_post(author_id, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "post": post })),
    ))
}

pub async fn update_post(
    Path(id): Path<Uuid>,
    State(service): State<MockBlogService>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Get author_id from authenticated user
    let author_id = Uuid::new_v4();
    let post = service.update_post(id, author_id, req).await?;

    Ok(Json(serde_json::json!({ "post": post })))
}

pub async fn delete_post(
    Path(id): Path<Uuid>,
    State(service): State<MockBlogService>,
) -> Result<StatusCode> {
    // TODO: Get author_id from authenticated user
    let author_id = Uuid::new_v4();
    service.delete_post(id, author_id).await?;

    Ok(StatusCode::NO_CONTENT)
}