use crate::mid::auth::auth;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use common::db::init_db_pool;
use time::macros::{format_description, offset};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{self, fmt, fmt::time::OffsetTime};

mod common;
mod mid;
mod model;
mod route;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_db_pool().await.expect("connection db error");

    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    // 日期格式化
    let timer = OffsetTime::new(
        offset!(+8),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    let f = fmt().event_format(
        fmt::format()
            .with_ansi(false)
            .with_timer(timer)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_line_number(true)
            .compact(),
    );
    // release 输出到日志文件
    if cfg!(debug_assertions) {
        f.init();
    } else {
        f.with_writer(non_blocking_appender).init();
    }

    let router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/login", post(route::user::login))
        .nest(
            "/user",
            route::user::router().route_layer(middleware::from_fn(auth)),
        )
        .layer(middleware::from_fn(mid::api_log::log))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
