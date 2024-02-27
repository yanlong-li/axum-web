use axum::Router;
use axum::routing::get;

use crate::controllers;

pub fn create_routes() -> Router {
    Router::new().route("/http/get", get(controllers::http::action_request))
}