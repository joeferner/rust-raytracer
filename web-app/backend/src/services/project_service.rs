use std::sync::Arc;

use anyhow::Result;

use crate::{
    repository::project_repository::{Project, ProjectRepository},
    routes::user_routes::AuthUser,
    services::examples_service::PROJECT_OWNER_EXAMPLE,
};

pub enum LoadProjectResult {
    Project(Project),
    NotFound,
    AccessDenied,
}

pub struct ProjectService {
    project_repository: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(project_repository: Arc<ProjectRepository>) -> Self {
        Self { project_repository }
    }

    pub async fn load_project(
        &self,
        project_id: &str,
        user: &AuthUser,
    ) -> Result<LoadProjectResult> {
        let project = self.project_repository.load(project_id).await?;
        match project {
            Some(project) => {
                if project.owner != user.email && project.owner != PROJECT_OWNER_EXAMPLE {
                    Ok(LoadProjectResult::AccessDenied)
                } else {
                    Ok(LoadProjectResult::Project(project))
                }
            }
            None => Ok(LoadProjectResult::NotFound),
        }
    }
}
