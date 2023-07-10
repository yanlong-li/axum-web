use std::net::SocketAddr;
use std::path::PathBuf;

use axum::{Router, routing::{get, post}};
use redis::Client;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::mysql::MySqlPool;

mod controllers;
mod schema;
mod entity;
mod forms;
mod services;
mod models;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env()
        //         .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        // )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // 创建一个连接池
    let pool = MySqlPool::connect("mysql://root:123456@localhost/axum")
        .await
        .unwrap();

    // 创建一个 Redis 客户端
    let client = Client::open("redis://127.0.0.1/").unwrap();


    // build our application with a route
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        // `GET /` goes to `root`
        .route("/", get(controllers::root))
        // `POST /users` goes to `create_user`
        .route("/users/:username", get(controllers::users::find_user))
        .route("/users", post(controllers::users::create_user))
        .route("/ws", get(controllers::ws::ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        ).layer(AddExtensionLayer::new(pool))
        .layer(AddExtensionLayer::new(client))
        ;

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



