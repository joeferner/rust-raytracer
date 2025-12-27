pub mod repository;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

use anyhow::Result;
use env_logger::Env;

use std::sync::Arc;

use routes::project::{
    __path_create_project, __path_get_project, __path_get_project_file, __path_get_projects,
    create_project, get_project, get_project_file, get_projects,
};
use routes::user::{
    __path_get_user_me, __path_google_token_verify, get_user_me, google_token_verify,
};
use tower_http::{cors, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

pub const PROJECT_TAG: &str = "project";
pub const USER_TAG: &str = "user";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = PROJECT_TAG, description = "Project management endpoints"),
        (name = USER_TAG, description = "User management endpoints"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
    let state = Arc::new(AppState::new().await?);
    let bind = state.settings.bind.clone();

    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_user_me))
        .routes(routes!(google_token_verify))
        .routes(routes!(get_project))
        .routes(routes!(get_projects))
        .routes(routes!(get_project_file))
        .routes(routes!(create_project))
        .with_state(state)
        .layer(cors)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    println!("listening http://{bind}");
    axum::serve(listener, router).await?;
    Ok(())
}
