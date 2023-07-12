use std::path::PathBuf;

use axum::Router;
use tower_http::services::ServeDir;

mod users;
mod ws;
mod root;
mod status;
mod test;

pub fn create_router() -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .merge(users::create_routes())
        .merge(ws::create_routes())
        .merge(root::create_routes())
        .merge(status::create_routes())
        .nest("/test", test::create_routes())
}