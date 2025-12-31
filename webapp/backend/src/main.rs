pub mod repository;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

use anyhow::Result;
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use clap::Parser;
use env_logger::Env;

use std::sync::Arc;

use log::info;
use routes::project_routes::{
    __path_copy_project, __path_create_project, __path_delete_project, __path_get_project,
    __path_get_project_file, __path_get_projects, copy_project, create_project, delete_project,
    get_project, get_project_file, get_projects,
};
use routes::user_routes::{
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Write OpenAPI/Swagger JSON to file and exit
    #[arg(long, value_name = "FILE")]
    write_swagger: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
    info!("Caustic Ray Tracer {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    // Check if we should write swagger and exit
    if let Some(output_path) = args.write_swagger {
        let (_, api) = build_api_router().split_for_parts();
        let json = serde_json::to_string_pretty(&api)?;
        std::fs::write(&output_path, json)?;
        println!("Swagger JSON written to: {}", output_path);
        return Ok(());
    }

    let state = Arc::new(AppState::new().await?);
    let bind = state.settings.bind.clone();

    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let (router, api) = build_api_router()
        .with_state(state)
        .layer(cors)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    println!("listening http://{bind}");
    axum::serve(listener, router).await?;
    Ok(())
}

fn build_api_router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_user_me))
        .routes(routes!(google_token_verify))
        .routes(routes!(get_project))
        .routes(routes!(get_projects))
        .routes(routes!(get_project_file))
        .routes(routes!(create_project))
        .routes(routes!(copy_project))
        .routes(routes!(delete_project))
        .layer(middleware::from_fn(access_logs))
}

async fn access_logs(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;

    info!("{} {} -> {}", method, uri, response.status());

    response
}
