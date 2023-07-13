use std::path::PathBuf;

use axum::Router;
use axum::routing::get_service;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod users;
mod ws;
mod root;
mod status;
mod test;
mod login;

pub fn create_router() -> Router {
    Router::new()
        .merge(users::create_routes())
        .merge(ws::create_routes())
        .merge(root::create_routes())
        .merge(status::create_routes())
        .merge(login::create_routes())
        .nest("/test", test::create_routes())
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static())
}


pub fn routes_static() -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/web");

    Router::new().nest_service("/static", get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true)))
}