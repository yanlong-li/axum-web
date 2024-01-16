use axum::Router;
use axum::routing::get;

use crate::controllers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/ws", get(controllers::ws::ws_handler))
}