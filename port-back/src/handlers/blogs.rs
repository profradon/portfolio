use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;
use serde::Deserialize;

use crate::models::{Blog, BlogResponse, CreateBlogRequest, UpdateBlogRequest};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// Public routes
pub async fn get_blogs(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<BlogResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Blog>("blogs");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "slug": 1,
            "excerpt": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blogs: Vec<BlogResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document::<BlogResponse>(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(blogs))
}

pub async fn get_blog(
    State(db): State<Database>,
    Path(slug): Path<String>,
) -> Result<Json<BlogResponse>, StatusCode> {
    let collection = db.collection::<Blog>("blogs");

    let pipeline = vec![
        doc! { "$match": { "slug": slug, "published": true } },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "slug": 1,
            "excerpt": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blogs: Vec<BlogResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document::<BlogResponse>(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if blogs.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(blogs.into_iter().next().unwrap()))
}

// Admin routes
pub async fn get_admin_blogs(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<BlogResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Blog>("blogs");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "slug": 1,
            "excerpt": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blogs: Vec<BlogResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(blogs))
}

pub async fn create_blog(
    State(db): State<Database>,
    Json(request): Json<CreateBlogRequest>,
) -> Result<Json<BlogResponse>, StatusCode> {
    let collection = db.collection::<Blog>("blogs");

    let now = chrono::Utc::now();
    let blog = Blog {
        id: None,
        title: request.title.clone(),
        slug: request.slug.clone(),
        excerpt: request.excerpt.clone(),
        content: request.content.clone(),
        tags: request.tags.clone(),
        published: request.published,
        created_at: now,
        updated_at: now,
    };

    let result = collection.insert_one(&blog).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.inserted_id.as_object_id().unwrap().to_hex();

    let response = BlogResponse {
        id,
        title: blog.title,
        slug: blog.slug,
        excerpt: blog.excerpt,
        content: blog.content,
        tags: blog.tags,
        published: blog.published,
        created_at: blog.created_at,
        updated_at: blog.updated_at,
    };

    Ok(Json(response))
}

pub async fn update_blog(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateBlogRequest>,
) -> Result<Json<BlogResponse>, StatusCode> {
    let collection = db.collection::<Blog>("blogs");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut update_doc = mongodb::bson::doc! { "updated_at": chrono::Utc::now().timestamp_millis() as i64 };

    if let Some(title) = request.title {
        update_doc.insert("title", title);
    }
    if let Some(slug) = request.slug {
        update_doc.insert("slug", slug);
    }
    if let Some(excerpt) = request.excerpt {
        update_doc.insert("excerpt", excerpt);
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

    // Fetch updated blog
    let pipeline = vec![
        doc! { "$match": { "_id": object_id } },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "slug": 1,
            "excerpt": 1,
            "content": 1,
            "tags": 1,
            "published": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blogs: Vec<BlogResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document::<BlogResponse>(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(blogs.into_iter().next().unwrap()))
}

pub async fn delete_blog(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let collection = db.collection::<Blog>("blogs");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = collection.delete_one(doc! { "_id": object_id }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}