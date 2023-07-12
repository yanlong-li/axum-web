use std::net::SocketAddr;

use axum::Router;
use redis::Client;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod schema;
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

    // 创建一个连接池
    let pool = databases::get_db().await;

    // 创建一个 Redis 客户端
    let client = Client::open("redis://127.0.0.1/").unwrap();


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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



