use std::sync::Arc;

use anyhow::{Context, Result};
use aws_sdk_s3::Client as S3Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    routes::user::AuthUser,
    utils::s3::{email_to_s3_key, read_json_from_s3, write_json_to_s3},
};

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub email: String,
    pub projects: Vec<UserDataProject>,
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

impl UserData {
    pub fn new(user: &AuthUser) -> Self {
        Self {
            email: user.email.to_owned(),
            projects: vec![],
        }
    }
}

pub struct UserRepository {
    client: Arc<S3Client>,
    bucket: String,
}

impl UserRepository {
    pub fn new(client: Arc<S3Client>, bucket: &str) -> Self {
        Self {
            client,
            bucket: bucket.to_owned(),
        }
    }

    pub async fn load(&self, user: &AuthUser) -> Result<Option<UserData>> {
        let bucket = &self.bucket;
        let key = self.get_user_key_from_auth_user(user);
        read_json_from_s3::<UserData>(&self.client, bucket, &key)
            .await
            .with_context(|| format!("loading user (user id: {})", user.email))
    }

    pub async fn save(&self, data: &UserData) -> Result<()> {
        let bucket = &self.bucket;
        let key = self.get_user_key_from_user_data(data);
        write_json_to_s3(&self.client, bucket, &key, data)
            .await
            .with_context(|| format!("saving user (user id: {})", data.email))
    }

    fn get_user_key_from_auth_user(&self, user: &AuthUser) -> String {
        format!("store/user/{}.json", email_to_s3_key(&user.email))
    }

    fn get_user_key_from_user_data(&self, data: &UserData) -> String {
        format!("store/user/{}.json", email_to_s3_key(&data.email))
    }
}
