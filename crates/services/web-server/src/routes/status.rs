use axum::Router;
use axum::routing::get;
use crate::controllers::status::action_status;

pub fn create_routes() -> Router {
    Router::new()
        .route("/status", get(action_status))
}