use std::env;
use std::net::SocketAddr;

use axum::Router;
use redis::Client;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod active_records;
mod services;
mod models;
mod routes;
mod middlewares;
mod databases;

mod utils;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();
    // 初始化环境
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 未定义");

    // 创建一个连接池
    let pool = databases::get_db(&database_url).await;

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL 未定义");

    // 创建一个 Redis 客户端
    let client = Client::open(redis_url).unwrap();


    // build our application with a route
    let app = Router::new()
        .merge(routes::create_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        ).layer(AddExtensionLayer::new(pool))
        .layer(AddExtensionLayer::new(client))
        ;

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // tracing::info!("listening on http://{}", listener);
    axum::serve::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



