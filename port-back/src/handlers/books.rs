use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{TimeZone, Utc};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;
use serde::Deserialize;

use crate::models::{Book, BookResponse, CreateBookRequest, UpdateBookRequest};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

fn bson_string(doc: &bson::Document, key: &str) -> Result<String, StatusCode> {
    match doc.get_str(key) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => {
            println!("BOOKS DESERIALIZATION ERROR: missing or invalid {}: {:?}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn bson_optional_string(doc: &bson::Document, key: &str) -> Result<Option<String>, StatusCode> {
    match doc.get(key) {
        Some(bson::Bson::String(s)) => Ok(Some(s.clone())),
        Some(bson::Bson::Null) | None => Ok(None),
        Some(other) => {
            println!("BOOKS DESERIALIZATION ERROR: invalid {} type: {:?}", key, other);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn bson_string_array(doc: &bson::Document, key: &str) -> Result<Vec<String>, StatusCode> {
    match doc.get_array(key) {
        Ok(array) => Ok(array
            .iter()
            .filter_map(|item| item.as_str().map(|s| s.to_string()))
            .collect()),
        Err(_) => Ok(vec![]),
    }
}

fn bson_optional_f32(doc: &bson::Document, key: &str) -> Result<Option<f32>, StatusCode> {
    match doc.get(key) {
        Some(bson::Bson::Double(value)) => Ok(Some(*value as f32)),
        Some(bson::Bson::Int32(value)) => Ok(Some(*value as f32)),
        Some(bson::Bson::Int64(value)) => Ok(Some(*value as f32)),
        Some(bson::Bson::Null) | None => Ok(None),
        Some(other) => {
            println!("BOOKS DESERIALIZATION ERROR: invalid {} type: {:?}", key, other);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn bson_datetime(doc: &bson::Document, key: &str) -> Result<chrono::DateTime<Utc>, StatusCode> {
    match doc.get(key) {
        Some(bson::Bson::DateTime(dt)) => Ok(Utc.timestamp_millis_opt(dt.timestamp_millis()).single().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())),
        Some(bson::Bson::Int64(ts)) => Ok(Utc.timestamp_millis_opt(*ts).single().unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())),
        Some(other) => {
            println!("BOOKS DESERIALIZATION ERROR: invalid {} type: {:?}", key, other);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        None => {
            println!("BOOKS DESERIALIZATION ERROR: missing {} field", key);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn parse_book_doc(doc: bson::Document) -> Result<BookResponse, StatusCode> {
    Ok(BookResponse {
        id: bson_string(&doc, "id")?,
        title: bson_string(&doc, "title")?,
        author: bson_string(&doc, "author")?,
        description: bson_optional_string(&doc, "description")?,
        isbn: bson_optional_string(&doc, "isbn")?,
        cover_url: bson_optional_string(&doc, "cover_url")?,
        rating: bson_optional_f32(&doc, "rating")?,
        review: bson_optional_string(&doc, "review")?,
        tags: bson_string_array(&doc, "tags")?,
        created_at: bson_datetime(&doc, "created_at")?,
        updated_at: bson_datetime(&doc, "updated_at")?,
    })
}

// Public routes
pub async fn get_books(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<BookResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<bson::Document>("books");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "author": 1,
            "description": 1,
            "isbn": 1,
            "cover_url": 1,
            "rating": 1,
            "review": 1,
            "tags": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|e| {
            println!("BOOKS DB AGGREGATE ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let books: Vec<BookResponse> = cursor
        .map(|doc_result| match doc_result {
            Ok(doc) => parse_book_doc(doc),
            Err(e) => {
                println!("BOOKS CURSOR ERROR: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
        .try_collect()
        .await
        .map_err(|e| {
            println!("BOOKS TRY_COLLECT ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(books))
}

// Admin routes
pub async fn get_admin_books(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<BookResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<bson::Document>("books");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "author": 1,
            "description": 1,
            "isbn": 1,
            "cover_url": 1,
            "rating": 1,
            "review": 1,
            "tags": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|e| {
            println!("BOOKS ADMIN DB AGGREGATE ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let books: Vec<BookResponse> = cursor
        .map(|doc_result| match doc_result {
            Ok(doc) => parse_book_doc(doc),
            Err(e) => {
                println!("BOOKS ADMIN CURSOR ERROR: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
        .try_collect()
        .await
        .map_err(|e| {
            println!("BOOKS ADMIN TRY_COLLECT ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(books))
}

pub async fn create_book(
    State(db): State<Database>,
    Json(request): Json<CreateBookRequest>,
) -> Result<Json<BookResponse>, StatusCode> {
    let collection = db.collection::<Book>("books");

    let now = chrono::Utc::now();
    let book = Book {
        id: None,
        title: request.title,
        author: request.author,
        description: request.description,
        isbn: request.isbn,
        cover_url: request.cover_url,
        rating: request.rating,
        review: request.review,
        tags: request.tags,
        created_at: now,
        updated_at: now,
    };

    let result = collection.insert_one(&book).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.inserted_id.as_object_id().unwrap().to_hex();

    let response = BookResponse {
        id,
        title: book.title,
        author: book.author,
        description: book.description,
        isbn: book.isbn,
        cover_url: book.cover_url,
        rating: book.rating,
        review: book.review,
        tags: book.tags,
        created_at: book.created_at,
        updated_at: book.updated_at,
    };

    Ok(Json(response))
}

pub async fn update_book(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateBookRequest>,
) -> Result<Json<BookResponse>, StatusCode> {
    let collection = db.collection::<Book>("books");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut update_doc = mongodb::bson::doc! { "updated_at": chrono::Utc::now().timestamp_millis() as i64 };

    if let Some(title) = request.title {
        update_doc.insert("title", title);
    }
    if let Some(author) = request.author {
        update_doc.insert("author", author);
    }
    if let Some(description) = request.description {
        update_doc.insert("description", description);
    }
    if let Some(isbn) = request.isbn {
        update_doc.insert("isbn", isbn);
    }
    if let Some(cover_url) = request.cover_url {
        update_doc.insert("cover_url", cover_url);
    }
    if let Some(rating) = request.rating {
        update_doc.insert("rating", rating);
    }
    if let Some(review) = request.review {
        update_doc.insert("review", review);
    }
    if let Some(tags) = request.tags {
        update_doc.insert("tags", tags);
    }

    let update = doc! { "$set": update_doc };

    let result = collection.update_one(doc! { "_id": object_id }, update).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.modified_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Fetch updated book
    let pipeline = vec![
        doc! { "$match": { "_id": object_id } },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "author": 1,
            "description": 1,
            "isbn": 1,
            "cover_url": 1,
            "rating": 1,
            "review": 1,
            "tags": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|e| {
            println!("BOOKS UPDATE DB AGGREGATE ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let books: Vec<BookResponse> = cursor
        .map(|doc_result| match doc_result {
            Ok(doc) => parse_book_doc(doc),
            Err(e) => {
                println!("BOOKS UPDATE CURSOR ERROR: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
        .try_collect()
        .await
        .map_err(|e| {
            println!("BOOKS UPDATE TRY_COLLECT ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(books.into_iter().next().unwrap()))
}

pub async fn delete_book(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let collection = db.collection::<Book>("books");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = collection.delete_one(doc! { "_id": object_id }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}