use std::sync::Arc;

use crate::{
    repository::{project_repository::ProjectRepository, user_repository::UserRepository},
    services::examples_service::ExampleService,
};
use anyhow::Result;
use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_s3::Client as S3Client;
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
    pub data_bucket: String,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<AppStateSettings>,
    pub project_repository: Arc<ProjectRepository>,
    pub user_repository: Arc<UserRepository>,
    pub example_service: Arc<ExampleService>,
}

fn default_bind() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_jwt_expire_duration_hours() -> u32 {
    30 * 24 // 30 days
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        dotenvy::dotenv()?;

        let settings = Arc::new(envy::prefixed("RAYTRACE_").from_env::<AppStateSettings>()?);

        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::defaults(BehaviorVersion::v2025_08_07())
            .region(region_provider)
            .load()
            .await;
        let s3_client = Arc::new(S3Client::new(&config));

        let project_repository = Arc::new(ProjectRepository::new(
            s3_client.clone(),
            &settings.data_bucket,
        ));
        let user_repository = Arc::new(UserRepository::new(s3_client, &settings.data_bucket));

        let example_service = Arc::new(ExampleService::new(project_repository.clone()).await?);

        Ok(AppState {
            settings,
            project_repository,
            user_repository,
            example_service,
        })
    }
}
