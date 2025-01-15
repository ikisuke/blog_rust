use axum::{
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::{ApiError, Result},
    middleware::Claims,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
}

async fn login(Json(req): Json<LoginRequest>) -> Result<Json<serde_json::Value>> {
    // TODO: Implement actual authentication logic with auth service
    // This is a mock implementation
    let user_id = Uuid::new_v4();
    let claims = Claims {
        sub: user_id,
        email: req.email,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string())
                .as_bytes(),
        ),
    )
    .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to create token")))?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user_id,
            "email": req.email
        }
    })))
}

async fn register(Json(req): Json<RegisterRequest>) -> Result<Json<serde_json::Value>> {
    // TODO: Implement actual registration logic with auth service
    // This is a mock implementation
    let user_id = Uuid::new_v4();
    let claims = Claims {
        sub: user_id,
        email: req.email.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string())
                .as_bytes(),
        ),
    )
    .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to create token")))?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user_id,
            "email": req.email,
            "username": req.username
        }
    })))
}