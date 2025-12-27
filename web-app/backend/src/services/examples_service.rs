use std::{fs, sync::Arc};

use crate::{
    repository::{
        project_repository::{Project, ProjectRepository},
        user_repository::UserDataProject,
    },
    utils::mime_type_from_path,
};
use anyhow::{Context, Result, anyhow};
use aws_sdk_s3::primitives::ByteStream;
use log::info;

pub const PROJECT_OWNER_EXAMPLE: &str = "example";

pub struct ExampleService {
    pub examples: Vec<UserDataProject>,
}

impl ExampleService {
    pub async fn new(project_repository: Arc<ProjectRepository>) -> Result<Self> {
        let mut me = Self { examples: vec![] };

        let files = fs::read_dir("examples")?;

        for file in files {
            let file = file.context("reading example file")?;
            me.update_project(project_repository.clone(), file).await?;
        }

        Ok(me)
    }

    async fn update_project(
        &mut self,
        project_repository: Arc<ProjectRepository>,
        example_dir: fs::DirEntry,
    ) -> Result<()> {
        let path = example_dir.path().join("project.json");
        info!("updating example project {path:?}");
        let json_str = fs::read_to_string(&path)?;
        let project = serde_json::from_str::<Project>(&json_str)
            .with_context(|| format!("could not convert {path:?} to json"))?;

        if project.owner != PROJECT_OWNER_EXAMPLE {
            return Err(anyhow!(
                "example project owner must be {PROJECT_OWNER_EXAMPLE} for {path:?}"
            ));
        }

        project_repository.save(&project).await?;

        for project_file in project.files {
            let project_file_path = example_dir.path().join(&project_file.filename);
            let content_type = mime_type_from_path(&project_file.filename)?;
            let data = ByteStream::from_path(&project_file_path)
                .await
                .with_context(|| format!("read file {project_file_path:?}"))?;
            project_repository
                .save_file(&project.id, &project_file.filename, &content_type, data)
                .await?;

            self.examples.push(UserDataProject {
                id: project.id.to_owned(),
                last_modified: "2024-12-26T15:30:00Z".parse().unwrap(), // TODO set real modified date
                name: project.name.to_owned(),
                readonly: true,
            });
        }

        Ok(())
    }
}
