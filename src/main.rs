use std::sync::Arc;

use axum::{routing::get, Router};
use maa_backend::{init_logger, route::ark_level_handler::get_levels, AppState};

#[tokio::main]
async fn main() {
    let _guard = init_logger();

    #[allow(clippy::expect_used)]
    let app_state = AppState::new().await.expect("Failed to create app state");

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/arknights/level", get(get_levels))
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
