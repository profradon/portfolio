use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: Vec<String>,
    pub project_types: Vec<String>,
    pub languages: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub cover_url: Option<String>,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub tags: Vec<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thought {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: Option<String>,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct About {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: Option<String>,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogResponse {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: Vec<String>,
    pub project_types: Vec<String>,
    pub languages: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub cover_url: Option<String>,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtResponse {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutResponse {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

// Request types
#[derive(Debug, Deserialize)]
pub struct CreateBlogRequest {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBlogRequest {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: Vec<String>,
    pub project_types: Vec<String>,
    pub languages: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub project_types: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub cover_url: Option<String>,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub cover_url: Option<String>,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateThoughtRequest {
    pub title: Option<String>,
    pub content: String,
    pub tags: Vec<String>,
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateThoughtRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAboutRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub role: String,
}