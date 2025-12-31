use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, header},
    response::Response,
};
use chrono::Utc;
use log::{error, info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    PROJECT_TAG,
    repository::{
        project_repository::{CONTENT_TYPE_OPENSCAD, Project, ProjectFile},
        user_repository::{UserData, UserDataProject, UserRepository},
    },
    routes::user_routes::{AuthUser, MaybeAuthUser},
    services::project_service::{LoadProjectResult, ProjectService},
    state::AppState,
};

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
}

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyProjectRequest {
    project_id: String,
}

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectRequest {
    project_id: String,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectsResponse {
    pub projects: Vec<UserDataProject>,
}

async fn assert_load_project(
    project_service: &ProjectService,
    project_id: &str,
    user: &Option<AuthUser>,
) -> Result<Project, StatusCode> {
    match project_service.load_project(project_id, user).await {
        Ok(project) => match project {
            LoadProjectResult::Project(project) => Ok(project),
            LoadProjectResult::NotFound => Err(StatusCode::NOT_FOUND),
            LoadProjectResult::AccessDenied => Err(StatusCode::UNAUTHORIZED),
        },
        Err(err) => {
            error!("failed to load project (project id: {project_id}): {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn assert_load_project_owner(
    project_service: &ProjectService,
    project_id: &str,
    user: &Option<AuthUser>,
) -> Result<Project, StatusCode> {
    let project = assert_load_project(project_service, project_id, user).await?;
    if let Some(user) = user
        && project.owner_user_id == user.user_id
    {
        Ok(project)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn assert_load_user_data(
    user_repository: &UserRepository,
    user: &AuthUser,
) -> Result<UserData, StatusCode> {
    user_repository
        .find_by_user_id(&user.user_id)
        .await
        .map_err(|err| {
            error!("failed to load user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)
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
    user: MaybeAuthUser,
) -> Result<Json<GetProjectsResponse>, StatusCode> {
    let mut projects: Vec<UserDataProject> = vec![];

    if let Some(user) = user.user {
        let user_data = state
            .user_repository
            .find_by_user_id(&user.user_id)
            .await
            .map_err(|err| {
                error!("failed to load user data: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if let Some(user_data) = user_data {
            for project in user_data.projects {
                projects.push(project);
            }
        }
    }

    let example_projects = state
        .project_service
        .get_example_projects()
        .await
        .map_err(|err| {
            error!("failed to load example projects: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    for project in example_projects {
        projects.push(UserDataProject {
            id: project.id,
            name: project.name,
            last_modified: project.last_modified,
            readonly: true,
        });
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
    user: MaybeAuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    match assert_load_project(&state.project_service, &project_id, &user.user).await {
        Ok(project) => Ok(Json(project)),
        Err(err) => Err(err),
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
    user: MaybeAuthUser,
    Path((project_id, filename)): Path<(String, String)>,
) -> Result<Response, StatusCode> {
    let project = assert_load_project(&state.project_service, &project_id, &user.user).await?;
    let project_file = project.files.iter().find(|f| f.filename == filename);
    let project_file = if let Some(project_file) = project_file {
        project_file
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    let file_data = state
        .project_repository
        .load_project_file_data(&project_id, &filename)
        .await
        .map_err(|err| {
            error!("failed to load project file: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(file_data) = file_data {
        let body = Body::from(file_data);
        let mut response = Response::new(body);
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).map_err(
                |err| {
                    error!("failed to parse header value: {err:?}");
                    StatusCode::INTERNAL_SERVER_ERROR
                },
            )?,
        );

        response.headers_mut().insert(
            header::CONTENT_TYPE,
            project_file
                .content_type
                .parse()
                .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
        );

        Ok(response)
    } else {
        warn!(
            "found project file but missing file data (project_id: {project_id}, filename: {filename})"
        );
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
    let now = Utc::now();

    info!(
        "creating project (name: {}, user_id: {})",
        payload.name, user.user_id
    );

    assert_load_user_data(&state.user_repository, &user).await?;

    // create project
    let project_id = Uuid::new_v4().to_string();

    let mut project = Project {
        id: project_id.clone(),
        owner_user_id: user.user_id,
        name: payload.name.clone(),
        last_modified: Utc::now(),
        files: vec![],
    };
    state
        .project_repository
        .insert_or_update_project(
            &project.id,
            &project.name,
            &project.owner_user_id,
            &now,
            &now,
        )
        .await
        .map_err(|err| {
            error!("failed to save project: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // create project file
    let file = ProjectFile {
        filename: "main.scad".to_string(),
        content_type: CONTENT_TYPE_OPENSCAD.to_string(),
    };
    let contents = "".to_string().into_bytes();
    state
        .project_repository
        .insert_or_update_project_file(
            &project_id,
            &file.filename,
            &file.content_type,
            &now,
            &now,
            &contents,
        )
        .await
        .map_err(|err| {
            error!("failed to save project file: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    project.files.push(file);

    Ok(Json(project))
}

#[utoipa::path(
    delete,
    path = "/api/v1/project/copy",
    responses(
        (status = OK, body = Project),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<DeleteProjectRequest>,
) -> Result<(), StatusCode> {
    info!(
        "deleting project (project id: {}, user_id: {})",
        payload.project_id, user.user_id
    );

    assert_load_user_data(&state.user_repository, &user).await?;
    assert_load_project_owner(&state.project_service, &payload.project_id, &Some(user)).await?;

    state
        .project_repository
        .delete_project(&payload.project_id)
        .await
        .map_err(|err| {
            error!("failed to delete project files: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v1/project/copy",
    responses(
        (status = OK, body = Project),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn copy_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<CopyProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    let now = Utc::now();

    info!(
        "copying project (project id: {}, user_id: {})",
        payload.project_id, user.user_id
    );

    let user_data = assert_load_user_data(&state.user_repository, &user).await?;

    let existing_project =
        match assert_load_project(&state.project_service, &payload.project_id, &Some(user)).await {
            Ok(project) => project,
            Err(err) => return Err(err),
        };

    let mut new_project = Project {
        id: Uuid::new_v4().to_string(),
        name: format!("{} Copy", existing_project.name),
        owner_user_id: user_data.user_id.clone(),
        files: vec![],
        last_modified: Utc::now(),
    };
    state
        .project_repository
        .insert_or_update_project(
            &new_project.id,
            &new_project.name,
            &new_project.owner_user_id,
            &now,
            &now,
        )
        .await
        .map_err(|err| {
            error!("failed to load user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    for file in &existing_project.files {
        let data = state
            .project_repository
            .load_project_file_data(&existing_project.id, &file.filename)
            .await
            .map_err(|err| {
                error!(
                    "failed to read existing project file (project_id: {}, filename: {}): {err:?}",
                    existing_project.id, file.filename
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        let data = if let Some(data) = data {
            data
        } else {
            error!(
                "missing existing project file data (project_id: {}, filename: {})",
                existing_project.id, file.filename
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };

        state
            .project_repository
            .insert_or_update_project_file(
                &new_project.id,
                &file.filename,
                &file.content_type,
                &now,
                &now,
                &data,
            )
            .await
            .map_err(|err| {
                error!(
                    "failed to copy file (project_id: {}, filename: {}): {err:?}",
                    existing_project.id, file.filename
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        new_project.files.push(ProjectFile {
            filename: file.filename.to_owned(),
            content_type: file.content_type.to_owned(),
        });
    }

    Ok(Json(new_project))
}
