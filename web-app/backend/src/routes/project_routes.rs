use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, header},
    response::Response,
};
use chrono::Utc;
use log::{error, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    PROJECT_TAG,
    repository::{
        project_repository::{CONTENT_TYPE_OPENSCAD, PROJECT_OWNER_EXAMPLE, Project, ProjectFile},
        user_repository::{UserData, UserDataProject},
    },
    routes::user::AuthUser,
    state::AppState,
};

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectsResponse {
    pub projects: Vec<UserDataProject>,
}

#[utoipa::path(
    get,
    path = "/api/v1/project",
    responses(
        (status = OK, body = GetProjectsResponse),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_projects(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<GetProjectsResponse>, StatusCode> {
    let user_data = state.user_repository.load(&user).await.map_err(|err| {
        error!("failed to load user data: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut projects = match user_data {
        Some(user_data) => user_data.projects,
        None => vec![],
    };

    for example in &state.example_service.examples {
        projects.push(example.clone());
    }

    let response = GetProjectsResponse { projects };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/project/{project_id}",
    responses(
        (status = OK, body = Project),
        (status = NOT_FOUND),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    let project = state
        .project_repository
        .load(&project_id)
        .await
        .map_err(|err| {
            error!("failed to load project: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match project {
        Some(project) => {
            if project.owner != user.email && project.owner != PROJECT_OWNER_EXAMPLE {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                Ok(Json(project))
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/project/{project_id}/file/{filename}",
    responses(
        (status = OK, content_type = "application/octet-stream"),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_project_file(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path((project_id, filename)): Path<(String, String)>,
) -> Result<Response, StatusCode> {
    // verify access
    let _ = get_project(State(state.clone()), user, Path(project_id.clone())).await?;

    let file_data = state
        .project_repository
        .load_file(&project_id, &filename)
        .await
        .map_err(|err| {
            error!("failed to load project file: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(file_data) = file_data {
        let bytes = file_data.body.collect().await.map_err(|err| {
            error!("failed to load file data bytes: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let body = Body::from(bytes.into_bytes());
        let mut response = Response::new(body);
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).map_err(
                |err| {
                    error!("failed to parse header value: {err}");
                    StatusCode::INTERNAL_SERVER_ERROR
                },
            )?,
        );

        if let Some(content_type) = file_data.content_type {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                content_type
                    .parse()
                    .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
            );
        }

        Ok(response)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/project",
    responses(
        (status = OK, body = Project),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn create_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    info!(
        "creating project (name: {}, username: {})",
        payload.name, user.email
    );

    let mut user_data = state
        .user_repository
        .load(&user)
        .await
        .map_err(|err| {
            error!("failed to load user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .unwrap_or_else(|| UserData::new(&user));

    let project_id = Uuid::new_v4().to_string();

    let file = ProjectFile {
        filename: "main.scad".to_string(),
    };
    let project = Project {
        id: project_id.clone(),
        owner: user.email,
        name: payload.name.clone(),
        files: vec![file],
    };

    user_data.projects.push(UserDataProject {
        id: project_id,
        readonly: false,
        name: payload.name,
        last_modified: Utc::now(),
    });

    state
        .project_repository
        .save_file(
            &project.id,
            "main.scad",
            CONTENT_TYPE_OPENSCAD,
            ByteStream::from("".to_string().into_bytes()),
        )
        .await
        .map_err(|err| {
            error!("failed to save project file: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    state
        .project_repository
        .save(&project)
        .await
        .map_err(|err| {
            error!("failed to save project: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    state
        .user_repository
        .save(&user_data)
        .await
        .map_err(|err| {
            error!("failed to save user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(project))
}
