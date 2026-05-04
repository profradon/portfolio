use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;

use crate::models::{About, AboutResponse, UpdateAboutRequest};

// Public routes
pub async fn get_about(
    State(db): State<Database>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<About>("about");

    let pipeline = vec![
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "content": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let abouts: Vec<AboutResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If no about document exists, return a default one
    if abouts.is_empty() {
        let default_about = AboutResponse {
            id: "default".to_string(),
            title: Some("About".to_string()),
            content: "Welcome to my personal website.".to_string(),
            updated_at: chrono::Utc::now(),
        };
        return Ok(Json(default_about));
    }

    Ok(Json(abouts.into_iter().next().unwrap()))
}

// Admin routes
pub async fn get_admin_about(
    State(db): State<Database>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<About>("about");

    let pipeline = vec![
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "content": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let abouts: Vec<AboutResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If no about document exists, return a default one
    if abouts.is_empty() {
        let default_about = AboutResponse {
            id: "default".to_string(),
            title: Some("About".to_string()),
            content: "Welcome to my personal website.".to_string(),
            updated_at: chrono::Utc::now(),
        };
        return Ok(Json(default_about));
    }

    Ok(Json(abouts.into_iter().next().unwrap()))
}

pub async fn update_about(
    State(db): State<Database>,
    Json(request): Json<UpdateAboutRequest>,
) -> Result<Json<AboutResponse>, StatusCode> {
    let collection = db.collection::<bson::Document>("about");

    // Try to find existing about document
    let existing_doc = collection.find_one(doc! {}).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let now = chrono::Utc::now();

    if let Some(doc) = existing_doc {
        // Extract ObjectId from document
        let object_id = doc.get_object_id("_id")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Update existing document
        let mut update_doc = mongodb::bson::doc! { "updated_at": now.timestamp_millis() as i64 };

        if let Some(title) = request.title.clone() {
            update_doc.insert("title", title);
        }
        if let Some(content) = request.content.clone() {
            update_doc.insert("content", content);
        }

        let update = doc! { "$set": update_doc };

        collection.update_one(doc! { "_id": object_id }, update).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let response = AboutResponse {
            id: object_id.to_hex(),
            title: request.title.clone(),
            content: request.content.clone().unwrap_or_else(|| {
                doc.get_str("content").unwrap_or("").to_string()
            }),
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