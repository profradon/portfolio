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
    println!("DEBUG: Parsing user doc: {:?}", doc);

    let id = doc.get_object_id("_id")
        .map(|oid| oid.to_hex())
        .ok();

    println!("DEBUG: Parsed id: {:?}", id);

    let email = match doc.get_str("email") {
        Ok(s) => s.to_string(),
        Err(e) => {
            println!("DEBUG: Email parse error: {:?}, trying alternative methods", e);
            // Try other possible field names or types
            if let Some(bson::Bson::String(s)) = doc.get("email") {
                s.clone()
            } else {
                println!("DEBUG: Email field not found or invalid type");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    println!("DEBUG: Parsed email: {}", email);

    let password_hash = match doc.get_str("password_hash") {
        Ok(s) => s.to_string(),
        Err(e) => {
            println!("DEBUG: Password hash parse error: {:?}, trying alternative methods", e);
            if let Some(bson::Bson::String(s)) = doc.get("password_hash") {
                s.clone()
            } else {
                println!("DEBUG: Password hash field not found or invalid type");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    println!("DEBUG: Parsed password_hash length: {}", password_hash.len());

    let role = match doc.get_str("role") {
        Ok(s) => s.to_string(),
        Err(e) => {
            println!("DEBUG: Role parse error: {:?}, trying alternative methods", e);
            if let Some(bson::Bson::String(s)) = doc.get("role") {
                s.clone()
            } else {
                println!("DEBUG: Role field not found or invalid type, defaulting to 'user'");
                "user".to_string()
            }
        }
    };

    println!("DEBUG: Parsed role: {}", role);

    let created_at = match doc.get_datetime("created_at") {
        Ok(dt) => Utc.timestamp_millis_opt(dt.timestamp_millis()).single().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()),
        Err(e) => {
            println!("DEBUG: Created_at parse error: {:?}, using current time", e);
            Utc::now()
        }
    };

    println!("DEBUG: Parsed created_at: {:?}", created_at);

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
    println!("DEBUG: Login attempt for email: {}", request.email);

    let collection = db.collection::<bson::Document>("users");

    // Find user by email
    let filter = doc! { "email": &request.email };
    println!("DEBUG: Filter: {:?}", filter);

    let doc = collection
        .find_one(filter)
        .await
        .map_err(|e| {
            println!("DEBUG: Database find_one error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    println!("DEBUG: Found user document");

    let user = parse_user_doc(doc)?;

    println!("DEBUG: User parsed successfully: id={:?}, email={}", user.id, user.email);

    // Verify password
    let is_valid = match verify(&request.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => {
            println!("DEBUG: Password verification error: {:?}, password length: {}, hash length: {}", 
                    e, request.password.len(), user.password_hash.len());
            println!("DEBUG: Hash preview: {}...", &user.password_hash[..std::cmp::min(20, user.password_hash.len())]);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("DEBUG: Password verification result: {}", is_valid);

    if !is_valid {
        println!("DEBUG: Password verification failed");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create JWT token
    let token = create_token(&user)
        .map_err(|e| {
            println!("DEBUG: Token creation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    println!("DEBUG: Token created successfully");

    let user_response = UserResponse {
        id: user.id.unwrap_or_default(),
        email: user.email,
        role: user.role,
    };

    let response = LoginResponse {
        user: user_response,
        token,
    };

    println!("DEBUG: Login successful for user: {}", request.email);
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