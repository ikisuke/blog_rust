use axum::{extract::Json, http::StatusCode};

use crate::{
    error::Result,
    models::{AuthResponse, LoginRequest, RegisterRequest, ValidateTokenRequest},
    services::AuthService,
};

pub async fn register(
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    let service = AuthService::new();
    let (token, user) = service.register(req).await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse { token, user }),
    ))
}

pub async fn login(
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    let service = AuthService::new();
    let (token, user) = service.login(req).await?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse { token, user }),
    ))
}

pub async fn validate_token(
    Json(req): Json<ValidateTokenRequest>,
) -> Result<StatusCode> {
    let service = AuthService::new();
    service.validate_token(&req.token)?;
    Ok(StatusCode::OK)
}