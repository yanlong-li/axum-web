use axum::Router;
use axum::routing::post;

use crate::controllers::login::action_login;

pub fn create_routes() -> Router {
    Router::new().route("/login", post(action_login))
}