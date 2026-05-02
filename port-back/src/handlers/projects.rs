use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{self, doc, oid::ObjectId, DateTime as BsonDateTime},
    Database,
};
use serde::Deserialize;
use chrono::Utc;

use crate::models::{Project, ProjectResponse, CreateProjectRequest, UpdateProjectRequest};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// ---------------- HELPERS ----------------

fn parse_doc(doc: bson::Document) -> Result<ProjectResponse, StatusCode> {
    bson::from_document::<ProjectResponse>(doc)
        .map_err(|e| {
            println!("❌ DESERIALIZATION ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

// ---------------- PUBLIC ----------------

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

    let cursor = collection.aggregate(pipeline).await.map_err(|e| {
        println!("❌ DB ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let projects = cursor
        .map(|doc| match doc {
            Ok(d) => parse_doc(d),
            Err(e) => {
                println!("❌ CURSOR ERROR: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
        .try_collect()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(projects))
}

// ---------------- ADMIN ----------------

pub async fn get_admin_projects(
    State(db): State<Database>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ProjectResponse>>, StatusCode> {
    get_projects(State(db), Query(pagination)).await
}

// ---------------- CREATE ----------------

pub async fn create_project(
    State(db): State<Database>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let now = Utc::now();

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

    let result = collection.insert_one(&project).await.map_err(|e| {
        println!("❌ INSERT ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let id = result
        .inserted_id
        .as_object_id()
        .map(|id| id.to_hex())
        .unwrap_or_default();

    Ok(Json(ProjectResponse {
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
    }))
}

// ---------------- UPDATE ----------------

pub async fn update_project(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let object_id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut update_doc = doc! {
        "updated_at": BsonDateTime::from_chrono(Utc::now())
    };

    if let Some(v) = request.title { update_doc.insert("title", v); }
    if let Some(v) = request.description { update_doc.insert("description", v); }
    if let Some(v) = request.long_description { update_doc.insert("long_description", v); }
    if let Some(v) = request.technologies { update_doc.insert("technologies", v); }
    if let Some(v) = request.project_types { update_doc.insert("project_types", v); }
    if let Some(v) = request.languages { update_doc.insert("languages", v); }
    if let Some(v) = request.github_url { update_doc.insert("github_url", v); }
    if let Some(v) = request.live_url { update_doc.insert("live_url", v); }
    if let Some(v) = request.image_url { update_doc.insert("image_url", v); }
    if let Some(v) = request.featured { update_doc.insert("featured", v); }

    let result = collection
        .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
        .await
        .map_err(|e| {
            println!("❌ UPDATE ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.modified_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // fetch updated
    let doc = collection
        .aggregate(vec![
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
        ])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_collect::<Vec<bson::Document>>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project = doc.into_iter().next().ok_or(StatusCode::NOT_FOUND)?;
    let response = parse_doc(project)?;

    Ok(Json(response))
}

// ---------------- DELETE ----------------

pub async fn delete_project(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let collection = db.collection::<Project>("projects");

    let object_id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = collection
        .delete_one(doc! { "_id": object_id })
        .await
        .map_err(|e| {
            println!("❌ DELETE ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}