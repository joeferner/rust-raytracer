use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::google::google_verify_token;
use crate::{AppState, USER_TAG};

use axum::extract::FromRequestParts;
use reqwest::StatusCode;

use axum::http::request::Parts;
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header};
use log::error;

#[derive(ToSchema, Deserialize)]
pub struct GoogleVerifyRequest {
    token: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    token: String,
    claims: Claims,
}

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // subject (user id)
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // issued at timestamp
}

// JWT token extractor - automatically validates JWT from Authorization header
#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check for Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..];

        // Verify and decode JWT
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.settings.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            email: token_data.claims.email,
            name: token_data.claims.name,
            picture: token_data.claims.picture,
        })
    }
}

// Optional AuthUser - returns None if authentication fails
pub struct MaybeAuthUser {
    pub user: Option<AuthUser>,
}

impl FromRequestParts<Arc<AppState>> for MaybeAuthUser {
    type Rejection = std::convert::Infallible; // Never fails

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Try to extract AuthUser, but don't fail if it doesn't work
        let auth_user = AuthUser::from_request_parts(parts, state).await.ok();
        Ok(MaybeAuthUser { user: auth_user })
    }
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMeResponse {
    pub user: Option<User>,
    pub settings: Settings,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub google_client_id: String,
}

#[utoipa::path(get, path = "/api/v1/user/me", responses((status = OK, body = UserMeResponse)), tag = USER_TAG)]
pub async fn get_user_me(
    State(state): State<Arc<AppState>>,
    auth_user: MaybeAuthUser,
) -> Json<UserMeResponse> {
    let user = if let Some(user) = auth_user.user {
        Some(User {
            user_id: user.user_id,
            email: user.email,
            name: user.name,
            picture: user.picture,
        })
    } else {
        None
    };

    Json(UserMeResponse {
        user,
        settings: Settings {
            google_client_id: state.settings.google_client_id.clone(),
        },
    })
}

fn generate_jwt(claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/user/google/verify",
    responses(
        (status = OK, body = AuthResponse),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)),
    tag = USER_TAG
)]
pub async fn google_token_verify(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GoogleVerifyRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let info = google_verify_token(&payload.token).await.map_err(|err| {
        error!("failed to verify token: {err}");
        StatusCode::UNAUTHORIZED
    })?;

    let exp_duration = chrono::Duration::hours(state.settings.jwt_expire_duration_hours as i64);
    let now = chrono::Utc::now();
    let exp = now + exp_duration;

    let claims = Claims {
        sub: info.sub.clone(),
        email: info.email.clone(),
        name: info.name.clone(),
        picture: Some(info.picture.clone()),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let jwt_token = generate_jwt(&claims, &state.settings.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token: jwt_token,
        claims,
    }))
}
