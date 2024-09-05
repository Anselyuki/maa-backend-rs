use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let log_writer = tracing_appender::rolling::daily("TODO/somelogdir", "TODO/somelogfile");
    let (appender, _guard) = tracing_appender::non_blocking(log_writer);

    tracing_subscriber::fmt().with_writer(appender).init();

    let app = Router::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
