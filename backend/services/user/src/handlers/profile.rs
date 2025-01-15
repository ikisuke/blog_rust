use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{Result, UserError},
    models::UpdateProfileRequest,
    services::{UserService, MockUserService},
};

pub async fn get_profile(
    Path(id): Path<Uuid>,
    State(service): State<MockUserService>,
) -> Result<Json<serde_json::Value>> {
    let profile = service.get_profile(id).await?;
    Ok(Json(serde_json::json!({ "profile": profile })))
}

pub async fn update_profile(
    Path(id): Path<Uuid>,
    State(service): State<MockUserService>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<serde_json::Value>> {
    // Validate request
    req.validate()
        .map_err(|e| UserError::Validation(e.to_string()))?;

    let profile = service.update_profile(id, req).await?;
    Ok(Json(serde_json::json!({ "profile": profile })))
}