use axum::Router;
use axum::routing::get;

use crate::controllers::ip::action_ip_info;

pub fn create_routes() -> Router {
    Router::new()
        .route("/ip", get(action_ip_info))
}