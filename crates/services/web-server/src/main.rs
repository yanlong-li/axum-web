use std::env;
use std::net::SocketAddr;

use axum_session::{SessionConfig, SessionLayer, SessionRedisPool, SessionRedisSessionStore};
use listenfd::ListenFd;
use redis::Client;
use redis_pool::RedisPool;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib_core::model::store as databases;

mod controllers;
mod routes;
mod middlewares;

mod utils;


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

    // 初始化环境
    dotenv::dotenv().ok();
    // 创建数据库连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 未定义");
    let mysql_pool = databases::get_db(&database_url).await;
    tracing::debug!("MySQL connection to address {}", database_url);

    // 创建 Redis 客户端
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL 未定义");
    tracing::debug!("Redis connection to address {}", &redis_url);
    let redis_client = Client::open(redis_url).unwrap();

    let session_name = env::var("SESSION_NAME").unwrap_or("session".to_string());

    let redis_pool = RedisPool::from(redis_client.clone());
    let session_config = SessionConfig::default()
        .with_session_name(session_name);

    let session_redis_pool = SessionRedisPool::from(redis_pool);

    let session_store = SessionRedisSessionStore::new(Some(session_redis_pool), session_config).await.unwrap();


    // build our application with a route
    let app = routes::create_router()
        .layer(CompressionLayer::new())
        .layer(AddExtensionLayer::new(mysql_pool))
        .layer(AddExtensionLayer::new(redis_client))
        .layer(SessionLayer::new(session_store))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let mut listenfd = ListenFd::from_env();
    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        // Server::from_tcp(l).unwrap()
        tokio::net::TcpListener::from_std(l)
    } else {
        // Server::bind(&([127, 0, 0, 1], 3030).into())
        let service_addr = env::var("WEB_SERVICE_ADDRESS").expect("WEB_SERVICE_ADDRESS 未定义1");
        tokio::net::TcpListener::bind(&service_addr).await
    };

    // run it with hyper
    let listener = server.unwrap();
    tracing::info!("Web service listening on http://{}", listener.local_addr().unwrap());
    axum::serve::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



