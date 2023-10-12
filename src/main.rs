use std::{env, net::SocketAddr};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use time::macros::{format_description, offset};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{self, fmt, fmt::time::OffsetTime};

mod handler;
mod mid;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    // 日期格式化
    let timer = OffsetTime::new(
        offset!(+8),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    fmt()
        .event_format(
            fmt::format()
                .with_ansi(false)
                .with_timer(timer)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_line_number(true)
                .compact(),
        )
        // .with_writer(non_blocking_appender)
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/login", post(handler::user::login))
        .layer(middleware::from_fn(mid::api_log::print_request_response))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any));

    let port = env::var("WEB.PORT")
        .unwrap_or("8888".to_string())
        .parse::<u16>()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
