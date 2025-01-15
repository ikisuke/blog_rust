use axum::{extract::State, Json};

use crate::{
    error::Result,
    services::{BlogService, MockBlogService},
};

pub async fn list_tags(
    State(service): State<MockBlogService>,
) -> Result<Json<serde_json::Value>> {
    let tags = service.list_tags().await?;
    Ok(Json(serde_json::json!({ "tags": tags })))
}