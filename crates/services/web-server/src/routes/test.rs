use axum::routing::{get, Router};
use crate::controllers::cookie::action_cookie_language;

pub fn create_routes() -> Router {
    Router::new().route("/language", get(action_cookie_language))
}