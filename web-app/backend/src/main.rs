pub mod routes;
pub mod jwt;

use std::sync::Arc;

use routes::auth::{__path_google_auth_handler, google_auth_handler};
use routes::user::{__path_get_user_me, get_user_me};
use tower_http::{cors, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

pub const AUTH_TAG: &str = "auth";
pub const USER_TAG: &str = "user";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = AUTH_TAG, description = "Auth endpoints"),
        (name = USER_TAG, description = "User management endpoints")
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub redirect_url: String,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        google_client_id: "YOUR_CLIENT_ID".to_string(),
        google_client_secret: "YOUR_CLIENT_SECRET".to_string(),
        redirect_url: "http://localhost:5173".to_string(),
        jwt_secret: "my-secret".to_string(),
    });

    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_user_me, google_auth_handler))
        .with_state(state)
        .layer(cors)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening http://0.0.0.0:3000");
    axum::serve(listener, router).await.unwrap();
}
