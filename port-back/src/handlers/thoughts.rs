use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;
use serde::Deserialize;

use crate::models::{Thought, ThoughtResponse, CreateThoughtRequest, UpdateThoughtRequest};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// Public routes
pub async fn get_thoughts(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ThoughtResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Thought>("thoughts");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thoughts: Vec<ThoughtResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(thoughts))
}

// Admin routes
pub async fn get_admin_thoughts(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ThoughtResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Thought>("thoughts");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thoughts: Vec<ThoughtResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(thoughts))
}

pub async fn create_thought(
    State(db): State<Database>,
    Json(request): Json<CreateThoughtRequest>,
) -> Result<Json<ThoughtResponse>, StatusCode> {
    let collection = db.collection::<Thought>("thoughts");

    let now = chrono::Utc::now();
    let thought = Thought {
        id: None,
        title: request.title,
        content: request.content,
        tags: request.tags,
        published: request.published,
        created_at: now,
        updated_at: now,
    };

    let result = collection.insert_one(&thought).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.inserted_id.as_object_id().unwrap().to_hex();

    let response = ThoughtResponse {
        id,
        title: thought.title,
        content: thought.content,
        tags: thought.tags,
        published: thought.published,
        created_at: thought.created_at,
        updated_at: thought.updated_at,
    };

    Ok(Json(response))
}

pub async fn update_thought(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateThoughtRequest>,
) -> Result<Json<ThoughtResponse>, StatusCode> {
    let collection = db.collection::<Thought>("thoughts");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut update_doc = mongodb::bson::doc! { "updated_at": chrono::Utc::now().timestamp_millis() as i64 };

    if let Some(title) = request.title {
        update_doc.insert("title", title);
    }
    if let Some(content) = request.content {
        update_doc.insert("content", content);
    }
    if let Some(tags) = request.tags {
        update_doc.insert("tags", tags);
    }
    if let Some(published) = request.published {
        update_doc.insert("published", published);
    }

    let update = doc! { "$set": update_doc };

    let result = collection.update_one(doc! { "_id": object_id }, update).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.modified_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Fetch updated thought
    let pipeline = vec![
        doc! { "$match": { "_id": object_id } },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thoughts: Vec<ThoughtResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(thoughts.into_iter().next().unwrap()))
}

pub async fn delete_thought(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let collection = db.collection::<Thought>("thoughts");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = collection.delete_one(doc! { "_id": object_id }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}