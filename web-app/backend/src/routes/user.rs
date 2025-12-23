use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{USER_TAG, jwt::AuthUser};

#[derive(ToSchema, Debug, Serialize)]
pub struct UserMe {
    pub user_id: String,
    pub email: String,
    pub name: String,
}

#[utoipa::path(get, path = "/api/v1/user/me", responses((status = OK, body = UserMe)), tag = USER_TAG)]
pub async fn get_user_me(auth_user: AuthUser) -> Json<UserMe> {
    Json(UserMe {
        user_id: auth_user.user_id,
        name: auth_user.name,
        email: auth_user.email,
    })
}
