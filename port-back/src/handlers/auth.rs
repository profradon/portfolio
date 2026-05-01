use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, verify};
use mongodb::bson::doc;
use mongodb::Database;
use chrono::Utc;

use crate::auth::{create_token, Claims};
use crate::models::{LoginRequest, LoginResponse, User, UserResponse, SignupRequest};

pub async fn login(
    State(db): State<Database>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let collection = db.collection::<User>("users");

    // Find user by email
    let filter = doc! { "email": &request.email };
    let user = collection
        .find_one(filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = verify(&request.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create JWT token
    let token = create_token(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_response = UserResponse {
        id: user.id.unwrap_or_default(),
        email: user.email,
        role: user.role,
    };

    let response = LoginResponse {
        user: user_response,
        token,
    };

    Ok(Json(response))
}

pub async fn me(
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user_response = UserResponse {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
    };

    Ok(Json(user_response))
}

pub async fn signup(
    State(db): State<Database>,
    Json(request): Json<SignupRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Only allow profradon@gmail.com to signup
    if request.email != "profradon@gmail.com" {
        return Err(StatusCode::FORBIDDEN);
    }

    let collection = db.collection::<User>("users");

    // Check if user already exists
    let filter = doc! { "email": &request.email };
    if collection
        .find_one(filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some()
    {
        return Err(StatusCode::CONFLICT); // User already exists
    }

    // Hash password
    let hashed_password = hash(&request.password, 10)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create new user
    let new_user = User {
        id: None,
        email: request.email,
        password_hash: hashed_password,
        role: "admin".to_string(),
        created_at: Utc::now(),
    };

    let result = collection
        .insert_one(&new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = result.inserted_id.as_object_id()
        .map(|id| id.to_hex())
        .unwrap_or_default();

    let user = User {
        id: Some(user_id),
        email: new_user.email,
        password_hash: new_user.password_hash,
        role: new_user.role,
        created_at: new_user.created_at,
    };

    // Create JWT token
    let token = create_token(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_response = UserResponse {
        id: user.id.unwrap_or_default(),
        email: user.email,
        role: user.role,
    };

    let response = LoginResponse {
        user: user_response,
        token,
    };

    Ok(Json(response))
}