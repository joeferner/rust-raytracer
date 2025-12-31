use std::{path::PathBuf, sync::Arc};

use crate::{
    repository::{
        create_db_pool, project_repository::ProjectRepository, user_repository::UserRepository,
    },
    services::{project_service::ProjectService, user_service::UserService},
};
use anyhow::Result;
use dotenvy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppStateSettings {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub jwt_secret: String,
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_jwt_expire_duration_hours")]
    pub jwt_expire_duration_hours: u32,
    pub sqlite_connection_string: String,
    pub data_path: PathBuf,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<AppStateSettings>,
    pub project_repository: Arc<ProjectRepository>,
    pub user_repository: Arc<UserRepository>,
    pub project_service: Arc<ProjectService>,
    pub user_service: Arc<UserService>,
}

fn default_bind() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_jwt_expire_duration_hours() -> u32 {
    30 * 24 // 30 days
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        dotenvy::dotenv().ok();

        let settings = Arc::new(envy::prefixed("RAYTRACE_").from_env::<AppStateSettings>()?);

        let db_pool = create_db_pool(&settings.sqlite_connection_string).await?;

        let project_repository =
            Arc::new(ProjectRepository::new(db_pool.clone(), &settings.data_path));
        let user_repository = Arc::new(UserRepository::new(db_pool));

        let user_service = Arc::new(UserService::new(user_repository.clone()));

        let project_service = Arc::new(ProjectService::new(project_repository.clone()));

        Ok(AppState {
            settings,
            project_repository,
            user_repository,
            user_service,
            project_service,
        })
    }
}
