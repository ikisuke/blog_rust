use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    error::Result,
    middleware::AuthUser,
    services::{mock::MockUserService, UserService},
};

pub fn router() -> Router {
    let user_service = MockUserService;

    Router::new()
        .route("/users/:username", get(get_user_by_username))
        .route("/users/:id", put(update_user))
        .with_state(user_service)
}

async fn get_user_by_username(
    Path(username): Path<String>,
    State(service): State<MockUserService>,
) -> Result<Json<serde_json::Value>> {
    let user = service.get_user_by_username(&username).await?;
    Ok(Json(serde_json::json!({ "user": user })))
}

async fn update_user(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    State(service): State<MockUserService>,
    Json(user): Json<crate::types::User>,
) -> Result<Json<serde_json::Value>> {
    // Verify that the authenticated user is updating their own profile
    if auth.id != id {
        return Err(crate::error::ApiError::Forbidden);
    }

    let updated_user = service.update_user(id, user).await?;
    Ok(Json(serde_json::json!({ "user": updated_user })))
}