use axum::{routing::get, Router};
use maa_backend::envs::{log_dir, log_prefix};
use tracing_appender::non_blocking::WorkerGuard;

#[tokio::main]
async fn main() {
    let _guard = init_logger();

    let app = Router::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// We expect these env vars to be set at runtime so we can call expect here
#[allow(clippy::expect_used)]
fn init_logger() -> WorkerGuard {
    let log_dir = log_dir().expect("LOG_DIR is not set");
    let log_prefix = log_prefix().expect("LOG_PREFIX is not set");

    let log_writer = tracing_appender::rolling::daily(log_dir, log_prefix);
    let (appender, guard) = tracing_appender::non_blocking(log_writer);

    tracing_subscriber::fmt().with_writer(appender).init();

    guard
}
