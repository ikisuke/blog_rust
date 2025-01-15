use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{CommentError, Result},
    models::{
        CommentFilters, CreateCommentRequest, ModerateCommentRequest, PaginationParams,
        UpdateCommentRequest,
    },
    services::{CommentService, MockCommentService},
};

pub async fn list_comments(
    Path(post_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<CommentFilters>,
    State(service): State<MockCommentService>,
) -> Result<Json<serde_json::Value>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);

    let response = service
        .list_comments(post_id, page, per_page, Some(filters))
        .await?;

    Ok(Json(serde_json::json!({
        "comments": response.comments,
        "pagination": {
            "total": response.total,
            "page": response.page,
            "per_page": response.per_page,
            "total_pages": response.total_pages
        }
    })))
}

pub async fn get_comment(
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
) -> Result<Json<serde_json::Value>> {
    let comment = service.get_comment(id).await?;
    Ok(Json(serde_json::json!({ "comment": comment })))
}

pub async fn create_comment(
    State(service): State<MockCommentService>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    // Validate request
    req.validate()
        .map_err(|e| CommentError::Validation(e.to_string()))?;

    // TODO: Get author_id from authenticated user
    let author_id = None;
    let comment = service.create_comment(author_id, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "comment": comment })),
    ))
}

pub async fn update_comment(
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<Json<serde_json::Value>> {
    // Validate request
    req.validate()
        .map_err(|e| CommentError::Validation(e.to_string()))?;

    // TODO: Get author_id from authenticated user
    let author_id = None;
    let comment = service.update_comment(id, author_id, req).await?;

    Ok(Json(serde_json::json!({ "comment": comment })))
}

pub async fn delete_comment(
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
) -> Result<StatusCode> {
    // TODO: Get author_id from authenticated user
    let author_id = None;
    service.delete_comment(id, author_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn moderate_comment(
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
    Json(req): Json<ModerateCommentRequest>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Get moderator_id from authenticated user and verify moderator role
    let moderator_id = Uuid::new_v4();
    let comment = service.moderate_comment(id, moderator_id, req).await?;

    Ok(Json(serde_json::json!({ "comment": comment })))
}

pub async fn list_replies(
    Path(id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
    State(service): State<MockCommentService>,
) -> Result<Json<serde_json::Value>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);

    let response = service.list_replies(id, page, per_page).await?;

    Ok(Json(serde_json::json!({
        "comments": response.comments,
        "pagination": {
            "total": response.total,
            "page": response.page,
            "per_page": response.per_page,
            "total_pages": response.total_pages
        }
    })))
}

pub async fn create_reply(
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
    Json(mut req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    // Set parent_id to the comment being replied to
    req.parent_id = Some(id);

    // Validate request
    req.validate()
        .map_err(|e| CommentError::Validation(e.to_string()))?;

    // TODO: Get author_id from authenticated user
    let author_id = None;
    let comment = service.create_comment(author_id, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "comment": comment })),
    ))
}