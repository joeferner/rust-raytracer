use std::sync::Arc;

use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{AUTH_TAG, AppState, jwt::Claims};

#[derive(ToSchema, Deserialize)]
pub struct GoogleTokenRequest {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    id_token: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: String,
    picture: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: GoogleUserInfo,
}

async fn exchange_code_for_token(
    state: &Arc<AppState>,
    code: &str,
) -> Result<GoogleTokenResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = [
        ("code", code),
        ("client_id", &state.google_client_id),
        ("client_secret", &state.google_client_secret),
        ("redirect_uri", &state.redirect_url),
        ("grant_type", "authorization_code"),
    ];

    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?;

    let token_response: GoogleTokenResponse = res.json().await?;
    Ok(token_response)
}

async fn get_user_info(access_token: &str) -> Result<GoogleUserInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?;

    let user_info: GoogleUserInfo = res.json().await?;
    Ok(user_info)
}

fn generate_jwt(
    user: &GoogleUserInfo,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(24); // Token expires in 24 hours

    let claims = Claims {
        sub: user.id.clone(),
        email: user.email.clone(),
        name: user.name.clone(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

#[utoipa::path(post, path = "/api/v1/auth/google", responses((status = OK, body = GoogleTokenRequest), (status = UNAUTHORIZED), (status = INTERNAL_SERVER_ERROR)), tag = AUTH_TAG)]
pub async fn google_auth_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GoogleTokenRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Exchange authorization code for access token
    let token_response = exchange_code_for_token(&state, &payload.code)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Get user info from Google
    let user_info = get_user_info(&token_response.access_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Generate JWT token
    let jwt_token = generate_jwt(&user_info, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token: jwt_token,
        user: user_info,
    }))
}
