use log::error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoogleTokenInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: String,
    pub name: String,
    pub picture: String,
    pub aud: String,
}

pub async fn google_verify_token(
    code: &str,
) -> Result<GoogleTokenInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", code)])
        .send()
        .await?;

    let text = response.text().await?;

    match serde_json::from_str::<GoogleTokenInfo>(&text) {
        Ok(info) => Ok(info),
        Err(err) => {
            error!("failed to decode GoogleTokenInfo: {text}");
            Err(Box::new(err))
        }
    }
}
