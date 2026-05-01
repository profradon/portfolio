use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: usize,  // expiration time
}

pub fn create_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone().unwrap_or_default(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    match verify_token(token) {
        Ok(claims) => {
            // Add user claims to request extensions for handlers to use
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}