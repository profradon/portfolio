use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::TimeZone;
use mongodb::bson::{self, doc, Bson};
use mongodb::Database;

use crate::models::{About, AboutResponse, UpdateAboutRequest};

fn parse_about_document(doc: bson::Document) -> AboutResponse {
    let id = match doc.get("_id") {
        Some(Bson::ObjectId(oid)) => oid.to_hex(),
        Some(Bson::String(s)) => s.clone(),
        _ => "default".to_string(),
    };

    let title = doc.get_str("title").ok().map(|s| s.to_string());
    let content = doc
        .get_str("content")
        .unwrap_or("Welcome to my personal website.")
        .to_string();

    let updated_at = match doc.get("updated_at") {
        Some(Bson::DateTime(dt)) => chrono::Utc.timestamp_millis_opt((*dt).timestamp_millis()).single().unwrap_or_else(|| chrono::Utc::now()),
        Some(Bson::Int64(ms)) => chrono::Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(|| chrono::Utc::now()),
        Some(Bson::Int32(ms)) => chrono::Utc.timestamp_millis_opt(i64::from(*ms)).single().unwrap_or_else(|| chrono::Utc::now()),
        _ => chrono::Utc::now(),
    };

    AboutResponse {
        id,
        title,
        content,
        updated_at,
    }
}

fn default_about() -> AboutResponse {
    AboutResponse {
        id: "default".to_string(),
        title: Some("About".to_string()),
        content: "Welcome to my personal website.".to_string(),
        updated_at: chrono::Utc::now(),
    }
}

// Public routes
pub async fn get_about(
    State(db): State<Database>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<bson::Document>("about");

    let document = collection.find_one(doc! {}).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let about = document.map(parse_about_document).unwrap_or_else(default_about);
    Ok(Json(about))
}

// Admin routes
pub async fn get_admin_about(
    State(db): State<Database>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<bson::Document>("about");

    let document = collection.find_one(doc! {}).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let about = document.map(parse_about_document).unwrap_or_else(default_about);
    Ok(Json(about))
}

pub async fn update_about(
    State(db): State<Database>,
    Json(request): Json<UpdateAboutRequest>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<bson::Document>("about");

    let existing_doc = collection.find_one(doc! {}).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let now = chrono::Utc::now();

    if let Some(doc) = existing_doc {
        let id_value = doc.get("_id")
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .clone();

        let mut update_doc = mongodb::bson::doc! { "updated_at": now.timestamp_millis() as i64 };

        if let Some(title) = request.title.clone() {
            update_doc.insert("title", title);
        }
        if let Some(content) = request.content.clone() {
            update_doc.insert("content", content);
        }

        let update = doc! { "$set": update_doc };
        collection.update_one(doc! { "_id": id_value.clone() }, update).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let response = AboutResponse {
            id: match id_value {
                Bson::ObjectId(oid) => oid.to_hex(),
                Bson::String(s) => s,
                _ => "default".to_string(),
            },
            title: request.title.clone().or_else(|| doc.get_str("title").ok().map(|s| s.to_string())),
            content: request.content.clone().or_else(|| doc.get_str("content").ok().map(|s| s.to_string()))
                .unwrap_or_else(|| "Welcome to my personal website.".to_string()),
            updated_at: now,
        };

        Ok(Json(response))
    } else {
        // Create new document
        let about = About {
            id: None,
            title: request.title.clone().or(Some("About".to_string())),
            content: request.content.clone().unwrap_or_else(|| "Welcome to my personal website.".to_string()),
            updated_at: now,
        };

        let collection_typed = db.collection::<About>("about");
        let result = collection_typed.insert_one(&about).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        let response = AboutResponse {
            id,
            title: about.title,
            content: about.content,
            updated_at: about.updated_at,
        };

        Ok(Json(response))
    }
}