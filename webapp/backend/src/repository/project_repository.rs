use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::repository::DbPool;

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub owner_user_id: String,
    pub name: String,
    #[schema(value_type = String)]
    pub last_modified: DateTime<Utc>,
    pub files: Vec<ProjectFile>,
}

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, FromRow)]
struct ProjectProjectFileRow {
    pub project_id: String,
    pub project_owner_user_id: String,
    pub project_name: String,
    pub project_last_modified: String,
    pub project_file_filename: Option<String>,
    pub project_file_content_type: Option<String>,
}

pub struct ReadProjectFileData {
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

pub struct ProjectRepository {
    db_pool: DbPool,
    data_path: PathBuf,
}

pub const CONTENT_TYPE_OPENSCAD: &str = "application/x-openscad";

impl ProjectRepository {
    pub fn new(db_pool: DbPool, data_path: &Path) -> Self {
        Self {
            db_pool,
            data_path: data_path.to_path_buf(),
        }
    }

    pub async fn find_by_owner_user_id(&self, owner_user_id: &str) -> Result<Vec<Project>> {
        let rows = sqlx::query_as::<_, ProjectProjectFileRow>(
            r#"
            SELECT 
                p.project_id AS project_id,
                p.owner_user_id AS project_owner_user_id,
                p.name AS project_name,
                p.last_modified AS project_last_modified,
                pf.filename AS project_file_filename,
                pf.content_type AS project_file_content_type
            FROM caustic_project p
            LEFT JOIN caustic_project_file pf ON p.project_id = pf.project_id
            WHERE p.owner_user_id = ?
            ORDER BY p.last_modified DESC
            "#,
        )
        .bind(owner_user_id)
        .fetch_all(&self.db_pool)
        .await
        .context("Failed to read project with project files")?;

        project_project_file_rows_to_projects(rows)
    }

    pub async fn find_by_project_id(&self, project_id: &str) -> Result<Option<Project>> {
        let rows = sqlx::query_as::<_, ProjectProjectFileRow>(
            r#"
            SELECT 
                p.project_id AS project_id,
                p.owner_user_id AS project_owner_user_id,
                p.name AS project_name,
                p.last_modified AS project_last_modified,
                pf.filename AS project_file_filename,
                pf.content_type AS project_file_content_type
            FROM caustic_project p
            LEFT JOIN caustic_project_file pf ON p.project_id = pf.project_id
            WHERE p.project_id = ?
            ORDER BY p.last_modified DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(&self.db_pool)
        .await
        .context("Failed to read project with project files")?;

        let mut projects = project_project_file_rows_to_projects(rows)?;
        if projects.is_empty() {
            Ok(None)
        } else if projects.len() == 1 {
            Ok(projects.pop())
        } else {
            Err(anyhow!("expected 1 project but found {}", projects.len()))
        }
    }

    pub async fn load_project_file_data(
        &self,
        project_id: &str,
        filename: &str,
    ) -> Result<Option<Vec<u8>>> {
        let path = self.data_path.join(project_id).join(filename);
        if path.exists() {
            let contents = fs::read(&path).with_context(|| format!("loading file {path:?}"))?;
            Ok(Some(contents))
        } else {
            Ok(None)
        }
    }

    pub async fn insert_or_update_project(
        &self,
        project_id: &str,
        name: &str,
        owner_user_id: &str,
        created: &DateTime<Utc>,
        last_modified: &DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO caustic_project (
                project_id,
                name,
                owner_user_id,
                created,
                last_modified
            ) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(project_id)
        .bind(name)
        .bind(owner_user_id)
        .bind(created)
        .bind(last_modified)
        .execute(&self.db_pool)
        .await
        .context("Failed to insert or update project")?;
        Ok(())
    }

    pub async fn insert_or_update_project_file(
        &self,
        project_id: &str,
        filename: &str,
        content_type: &str,
        created: &DateTime<Utc>,
        last_modified: &DateTime<Utc>,
        data: &Vec<u8>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO caustic_project_file (
                project_id,
                filename,
                content_type,
                created,
                last_modified
            ) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(project_id)
        .bind(filename)
        .bind(content_type)
        .bind(created)
        .bind(last_modified)
        .execute(&self.db_pool)
        .await
        .context("Failed to insert or update project file")?;

        let project_path = self.data_path.join(project_id);
        fs::create_dir_all(&project_path)
            .with_context(|| format!("saving file {project_path:?} (could not create path)"))?;
        let path = project_path.join(filename);
        fs::write(&path, data).with_context(|| format!("saving file {path:?}"))?;
        Ok(())
    }

    pub async fn delete_project(&self, project_id: &str) -> Result<()> {
        let project_path = self.data_path.join(project_id);
        if fs::exists(&project_path)? {
            fs::remove_dir_all(&project_path)
                .with_context(|| format!("Failed to delete project directory {project_path:?}"))?;
        }

        sqlx::query("DELETE FROM caustic_project_file WHERE project_id = ?")
            .bind(project_id)
            .execute(&self.db_pool)
            .await
            .context("Failed to delete project files")?;

        sqlx::query("DELETE FROM caustic_project WHERE project_id = ?")
            .bind(project_id)
            .execute(&self.db_pool)
            .await
            .context("Failed to delete project")?;

        Ok(())
    }
}

fn project_project_file_rows_to_projects(rows: Vec<ProjectProjectFileRow>) -> Result<Vec<Project>> {
    let mut projects: HashMap<String, Project> = HashMap::new();

    for row in rows {
        let project = projects.entry(row.project_id.clone()).or_insert(Project {
            id: row.project_id,
            owner_user_id: row.project_owner_user_id,
            name: row.project_name,
            last_modified: row.project_last_modified.parse()?,
            files: vec![],
        });

        if let (Some(project_file_filename), Some(project_file_content_type)) =
            (row.project_file_filename, row.project_file_content_type)
        {
            project.files.push(ProjectFile {
                filename: project_file_filename,
                content_type: project_file_content_type,
            });
        }
    }

    Ok(projects.into_values().collect())
}
