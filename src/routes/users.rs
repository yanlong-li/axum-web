use axum::Router;
use axum::routing::{get, post};

use crate::controllers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/users/:username", get(controllers::users::action_find_user))
        .route("/users", post(controllers::users::action_create_user))
        .route("/users", get(controllers::users::action_list))
}