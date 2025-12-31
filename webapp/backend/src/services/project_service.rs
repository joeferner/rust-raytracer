use std::sync::Arc;

use anyhow::Result;

use crate::{
    repository::project_repository::{Project, ProjectRepository},
    routes::user_routes::AuthUser,
};

pub const PROJECT_EXAMPLE_OWNER_ID: &str = "examples";

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
        user: &Option<AuthUser>,
    ) -> Result<LoadProjectResult> {
        let project = self
            .project_repository
            .find_by_project_id(project_id)
            .await?;
        match project {
            Some(project) => {
                if project.owner_user_id == PROJECT_EXAMPLE_OWNER_ID {
                    Ok(LoadProjectResult::Project(project))
                } else if let Some(user) = &user
                    && project.owner_user_id == user.user_id
                {
                    Ok(LoadProjectResult::Project(project))
                } else {
                    Ok(LoadProjectResult::AccessDenied)
                }
            }
            None => Ok(LoadProjectResult::NotFound),
        }
    }

    pub async fn get_example_projects(&self) -> Result<Vec<Project>> {
        self.project_repository
            .find_by_owner_user_id(PROJECT_EXAMPLE_OWNER_ID)
            .await
    }
}
