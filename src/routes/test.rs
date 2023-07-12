use axum::routing::{get, Router};
use crate::controllers::test::action_cookie;

pub fn create_routes() -> Router {
    Router::new().route("/cookie", get(action_cookie))
}