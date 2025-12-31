use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

use crate::{
    repository::user_repository::{UserData, UserRepository},
    routes::user_routes::AuthUser,
};

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn load_or_create_user(&self, auth_user: &AuthUser) -> Result<UserData> {
        match self
            .user_repository
            .find_by_user_id(&auth_user.user_id)
            .await?
        {
            Some(user_data) => Ok(user_data),
            None => {
                let user_data = UserData {
                    user_id: auth_user.user_id.clone(),
                    email: auth_user.email.clone(),
                    created: Utc::now(),
                    projects: vec![],
                };
                self.user_repository.create(&user_data).await?;
                Ok(user_data)
            }
        }
    }
}
