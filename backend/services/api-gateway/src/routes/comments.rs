use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    error::Result,
    middleware::AuthUser,
    services::{mock::MockCommentService, CommentService},
    types::{CreateCommentRequest, PaginationParams, UpdateCommentRequest},
};

pub fn router() -> Router {
    let comment_service = MockCommentService;

    Router::new()
        .route("/posts/:post_id/comments", get(get_comments).post(create_comment))
        .route(
            "/comments/:id",
            put(update_comment).delete(delete_comment),
        )
        .with_state(comment_service)
}

async fn get_comments(
    Path(post_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
    State(service): State<MockCommentService>,
) -> Result<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let (comments, total) = service
        .get_comments(post_id, page, per_page, None)
        .await?;

    Ok(Json(serde_json::json!({
        "comments": comments,
        "pagination": {
            "total": total,
            "page": page,
            "per_page": per_page,
            "total_pages": (total as f64 / per_page as f64).ceil() as u32
        }
    })))
}

async fn create_comment(
    auth: Option<AuthUser>,
    Path(post_id): Path<Uuid>,
    State(service): State<MockCommentService>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<serde_json::Value>> {
    let author_id = auth.map(|user| user.id);
    let comment = service.create_comment(post_id, author_id, req).await?;
    Ok(Json(serde_json::json!({ "comment": comment })))
}

async fn update_comment(
    auth: Option<AuthUser>,
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<Json<serde_json::Value>> {
    let author_id = auth.map(|user| user.id);
    let comment = service.update_comment(id, author_id, req).await?;
    Ok(Json(serde_json::json!({ "comment": comment })))
}

async fn delete_comment(
    auth: Option<AuthUser>,
    Path(id): Path<Uuid>,
    State(service): State<MockCommentService>,
) -> Result<Json<serde_json::Value>> {
    let author_id = auth.map(|user| user.id);
    service.delete_comment(id, author_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}