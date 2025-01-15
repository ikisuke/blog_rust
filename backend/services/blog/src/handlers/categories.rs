use axum::{extract::State, Json};

use crate::{
    error::Result,
    services::{BlogService, MockBlogService},
};

pub async fn list_categories(
    State(service): State<MockBlogService>,
) -> Result<Json<serde_json::Value>> {
    let categories = service.list_categories().await?;
    Ok(Json(serde_json::json!({ "categories": categories })))
}