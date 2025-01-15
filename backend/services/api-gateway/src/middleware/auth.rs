use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Unauthorized)?;

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(
                std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "your-secret-key".to_string())
                    .as_bytes(),
            ),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized)?;

        Ok(AuthUser {
            id: token_data.claims.sub,
            email: token_data.claims.email,
        })
    }
}

pub async fn require_auth<B>(
    req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(ApiError::Unauthorized)?;

    let auth_header_str = auth_header
        .to_str()
        .map_err(|_| ApiError::Unauthorized)?;

    if !auth_header_str.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    let token = &auth_header_str["Bearer ".len()..];

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string())
                .as_bytes(),
        ),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(next.run(req).await)
}