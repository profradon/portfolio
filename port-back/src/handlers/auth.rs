use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, verify};
use mongodb::bson::{self, doc};
use mongodb::Database;
use chrono::{TimeZone, Utc};

use crate::auth::{create_token, Claims};
use crate::models::{LoginRequest, LoginResponse, UserResponse, SignupRequest};

// Helper function to parse User from BSON document
fn parse_user_doc(doc: bson::Document) -> Result<crate::models::User, StatusCode> {
    let id = doc.get_object_id("_id")
        .map(|oid| oid.to_hex())
        .ok();

    let email = doc.get_str("email")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let password_hash = doc.get_str("password_hash")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let role = doc.get_str("role")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let created_at = doc.get_datetime("created_at")
        .map(|dt| Utc.timestamp_millis_opt(dt.timestamp_millis()).single().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(crate::models::User {
        id,
        email,
        password_hash,
        role,
        created_at,
    })
}

pub async fn login(
    State(db): State<Database>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let collection = db.collection::<bson::Document>("users");

    // Find user by email
    let filter = doc! { "email": &request.email };
    let doc = collection
        .find_one(filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = parse_user_doc(doc)?;

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

    let collection = db.collection::<bson::Document>("users");

    // Check if any admin user already exists - only allow one-time signup
    let admin_filter = doc! { "role": "admin" };
    let existing_admin = collection
        .find_one(admin_filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_admin.is_some() {
        return Err(StatusCode::FORBIDDEN); // Admin already exists, signup disabled
    }

    // Check if user already exists (shouldn't happen but safety check)
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
    let now = Utc::now();
    let user_doc = doc! {
        "email": &request.email,
        "password_hash": &hashed_password,
        "role": "admin",
        "created_at": bson::DateTime::from_system_time(now.into())
    };

    let result = collection
        .insert_one(user_doc)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = result.inserted_id.as_object_id()
        .map(|id| id.to_hex())
        .unwrap_or_default();

    let user = crate::models::User {
        id: Some(user_id),
        email: request.email,
        password_hash: hashed_password,
        role: "admin".to_string(),
        created_at: now,
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