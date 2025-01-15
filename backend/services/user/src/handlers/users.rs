use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    error::Result,
    services::{UserService, MockUserService},
};

pub async fn get_user(
    Path(username): Path<String>,
    State(service): State<MockUserService>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Get current_user_id from authenticated user
    let current_user_id = None;
    let user = service.get_user_by_username(&username, current_user_id).await?;
    Ok(Json(serde_json::json!({ "user": user })))
}