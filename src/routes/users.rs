use axum::{middleware::{self, Next}, Router};
use axum::extract::Query;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::controllers;
use crate::models::auth::Token;

pub fn create_routes() -> Router {
    Router::new()
        .route("/users/:username", get(controllers::users::action_find_user))
        .route("/users", post(controllers::users::action_create_user))
        .route("/users", get(controllers::users::action_list))
        .route("/users/my", get(controllers::users::action_my_info)
            .layer(middleware::from_fn(check_auth)),
        )
}

async fn check_auth<B>(
    Query(token): Query<Token>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 检查状态是否为true
    match token.token {
        Some(_) => {
            // todo 这里可以对 token 进一步校验
            next.run(request).await
        }
        None => {
            // 如果不是，返回401错误码
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}