use std::sync::Arc;

use axum::{middleware::{self, Next}, Router};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::controllers;

pub fn create_routes() -> Router {
    let state = Arc::new(false);

    Router::new()
        .route("/users/:username", get(controllers::users::action_find_user))
        .route("/users", post(controllers::users::action_create_user))
        .route("/users", get(controllers::users::action_list))
        .route("/users/my", get(controllers::users::action_my_info)
            .layer(middleware::from_fn_with_state(state, check_state)),
        )
}

async fn check_state<B>(
    State(state): State<Arc<bool>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 检查状态是否为true
    if *state {
        // 如果是，继续执行下一个中间件或处理器
        next.run(request).await
    } else {
        // 如果不是，返回401错误码
        StatusCode::UNAUTHORIZED.into_response()
    }
}