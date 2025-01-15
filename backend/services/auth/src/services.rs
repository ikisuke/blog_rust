use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    error::{AuthError, Result},
    models::{Claims, LoginRequest, RegisterRequest, User, UserResponse},
};

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new() -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key".to_string());
        Self { jwt_secret }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<(String, UserResponse)> {
        // TODO: Check if user exists in database
        // This is a mock implementation
        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;

        let user = User {
            id: Uuid::new_v4(),
            email: req.email.clone(),
            username: req.username,
            password_hash,
            display_name: None,
            bio: None,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let token = self.create_token(&user)?;
        Ok((token, user.into()))
    }

    pub async fn login(&self, req: LoginRequest) -> Result<(String, UserResponse)> {
        // TODO: Get user from database
        // This is a mock implementation
        let stored_password_hash = hash("password123", DEFAULT_COST).unwrap();
        if !verify(req.password.as_bytes(), &stored_password_hash)
            .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to verify password: {}", e)))?
        {
            return Err(AuthError::InvalidCredentials);
        }

        let user = User {
            id: Uuid::new_v4(),
            email: req.email.clone(),
            username: "mockuser".to_string(),
            password_hash: stored_password_hash,
            display_name: None,
            bio: None,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let token = self.create_token(&user)?;
        Ok((token, user.into()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }

    fn create_token(&self, user: &User) -> Result<String> {
        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::Internal(anyhow::anyhow!("Failed to create token: {}", e)))
    }
}