use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::repository::DbPool;

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_id: String,
    pub email: String,
    pub projects: Vec<UserDataProject>,
    #[schema(value_type = String)]
    pub created: DateTime<Utc>,
}

#[derive(ToSchema, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataProject {
    pub id: String,
    pub name: String,
    pub readonly: bool,
    #[schema(value_type = String)]
    pub last_modified: DateTime<Utc>,
}

pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn find_by_user_id(&self, user_id: &str) -> Result<Option<UserData>> {
        #[derive(Debug, FromRow)]
        struct UserProjectRow {
            user_id: String,
            user_email: String,
            user_created: String,
            project_id: Option<String>,
            project_name: Option<String>,
            project_last_modified: Option<String>,
        }

        let rows = sqlx::query_as::<_, UserProjectRow>(
            r#"
            SELECT 
                u.user_id as user_id,
                u.email as user_email,
                u.created as user_created,
                p.project_id as project_id,
                p.name as project_name,
                p.last_modified as project_last_modified
            FROM caustic_user u
            LEFT JOIN caustic_project p ON u.user_id = p.owner_user_id
            WHERE u.user_id = ?
            ORDER BY p.last_modified DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .context("Failed to read user with projects")?;

        let mut users: HashMap<String, UserData> = HashMap::new();

        for row in rows {
            let user = users.entry(row.user_id.clone()).or_insert(UserData {
                user_id: row.user_id,
                created: row.user_created.parse()?,
                email: row.user_email,
                projects: vec![],
            });

            if let (Some(project_id), Some(project_name), Some(project_last_modified)) =
                (row.project_id, row.project_name, row.project_last_modified)
            {
                user.projects.push(UserDataProject {
                    id: project_id,
                    name: project_name,
                    readonly: false,
                    last_modified: project_last_modified.parse()?,
                });
            }
        }

        if users.is_empty() {
            Ok(None)
        } else if users.len() == 1 {
            Ok(users.remove(user_id))
        } else {
            Err(anyhow!(
                "expected 1 user but found {} (user_id: {})",
                users.len(),
                user_id
            ))
        }
    }

    pub async fn create(&self, data: &UserData) -> Result<()> {
        sqlx::query("INSERT INTO caustic_user (user_id, email, created) VALUES (?, ?, ?)")
            .bind(&data.user_id)
            .bind(&data.email)
            .bind(data.created)
            .execute(&self.db_pool)
            .await
            .context("Failed to insert user")?;

        Ok(())
    }
}
