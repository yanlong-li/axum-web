use axum::{middleware::{self}, Router};
use axum::routing::{get, post};

use crate::controllers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/users/:username", get(controllers::users::action_find_user))
        .route("/users", post(controllers::users::action_create_user))
        .route("/users", get(controllers::users::action_list))
        .route("/users/self", get(controllers::users::action_current_user))
        .route("/users/logout", get(controllers::users::action_logout))
        .route_layer(middleware::from_fn(crate::middlewares::auth::mw_require_auth))
}