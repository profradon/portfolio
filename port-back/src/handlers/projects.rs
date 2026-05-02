use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;
use serde::Deserialize;

use crate::models::{Project, ProjectResponse, CreateProjectRequest, UpdateProjectRequest};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// Public routes
pub async fn get_projects(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ProjectResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Project>("projects");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "description": 1,
            "long_description": 1,
            "technologies": 1,
            "project_types": 1,
            "languages": 1,
            "github_url": 1,
            "live_url": 1,
            "image_url": 1,
            "featured": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let projects: Vec<ProjectResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(projects))
}

// Admin routes
pub async fn get_admin_projects(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ProjectResponse>>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let skip = (page - 1) * limit;

    let collection = db.collection::<Project>("projects");

    let pipeline = vec![
        doc! { "$sort": { "created_at": -1 } },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "description": 1,
            "long_description": 1,
            "technologies": 1,
            "project_types": 1,
            "languages": 1,
            "github_url": 1,
            "live_url": 1,
            "image_url": 1,
            "featured": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let projects: Vec<ProjectResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(projects))
}

pub async fn create_project(
    State(db): State<Database>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let now = chrono::Utc::now();
    let project = Project {
        id: None,
        title: request.title,
        description: request.description,
        long_description: request.long_description,
        technologies: request.technologies,
        project_types: request.project_types,
        languages: request.languages,
        github_url: request.github_url,
        live_url: request.live_url,
        image_url: request.image_url,
        featured: request.featured,
        created_at: now,
        updated_at: now,
    };

    let result = collection.insert_one(&project).await.map_err(|err| {
        eprintln!("Failed to insert project: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let id = match result.inserted_id {
        bson::Bson::ObjectId(oid) => oid.to_hex(),
        other => other.to_string(),
    };

    let response = ProjectResponse {
        id,
        title: project.title,
        description: project.description,
        long_description: project.long_description,
        technologies: project.technologies,
        project_types: project.project_types,
        languages: project.languages,
        github_url: project.github_url,
        live_url: project.live_url,
        image_url: project.image_url,
        featured: project.featured,
        created_at: project.created_at,
        updated_at: project.updated_at,
    };

    Ok(Json(response))
}

pub async fn update_project(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut update_doc = mongodb::bson::doc! { "updated_at": chrono::Utc::now().timestamp_millis() as i64 };

    if let Some(title) = request.title {
        update_doc.insert("title", title);
    }
    if let Some(description) = request.description {
        update_doc.insert("description", description);
    }
    if let Some(long_description) = request.long_description {
        update_doc.insert("long_description", long_description);
    }
    if let Some(technologies) = request.technologies {
        update_doc.insert("technologies", technologies);
    }
    if let Some(project_types) = request.project_types {
        update_doc.insert("project_types", project_types);
    }
    if let Some(languages) = request.languages {
        update_doc.insert("languages", languages);
    }
    if let Some(github_url) = request.github_url {
        update_doc.insert("github_url", github_url);
    }
    if let Some(live_url) = request.live_url {
        update_doc.insert("live_url", live_url);
    }
    if let Some(image_url) = request.image_url {
        update_doc.insert("image_url", image_url);
    }
    if let Some(featured) = request.featured {
        update_doc.insert("featured", featured);
    }

    let update = doc! { "$set": update_doc };

    let result = collection.update_one(doc! { "_id": object_id }, update).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.modified_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Fetch updated project
    let pipeline = vec![
        doc! { "$match": { "_id": object_id } },
        doc! { "$project": {
            "_id": 0,
            "id": { "$toString": "$_id" },
            "title": 1,
            "description": 1,
            "long_description": 1,
            "technologies": 1,
            "project_types": 1,
            "languages": 1,
            "github_url": 1,
            "live_url": 1,
            "image_url": 1,
            "featured": 1,
            "created_at": 1,
            "updated_at": 1
        }}
    ];

    let cursor = collection.aggregate(pipeline).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let projects: Vec<ProjectResponse> = cursor
        .map(|doc_result| {
            match doc_result {
                Ok(doc) => bson::from_document(doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project = projects.into_iter().next().ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(project))
}

pub async fn delete_project(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = collection.delete_one(doc! { "_id": object_id }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}