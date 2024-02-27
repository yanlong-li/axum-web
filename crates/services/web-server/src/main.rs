use std::env;
use std::net::SocketAddr;

use axum_session::{SessionConfig, SessionLayer, SessionRedisPool, SessionRedisSessionStore};
use redis::Client;
use redis_pool::RedisPool;
use tower_http::add_extension::AddExtensionLayer;
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


    let redis_pool = RedisPool::from(redis_client.clone());
    let session_config = SessionConfig::default()
        .with_session_name("session");

    let session_redis_pool = SessionRedisPool::from(redis_pool);

    let session_store = SessionRedisSessionStore::new(Some(session_redis_pool), session_config).await.unwrap();


    // build our application with a route
    let app = routes::create_router()
        .layer(AddExtensionLayer::new(mysql_pool))
        .layer(AddExtensionLayer::new(redis_client))
        .layer(SessionLayer::new(session_store))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let server = env::var("WEB_SERVICE_ADDRESS").expect("WEB_SERVICE_ADDRESS 未定义");

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(&server).await.unwrap();
    tracing::info!("Web service listening on http://{}", server);
    axum::serve::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



