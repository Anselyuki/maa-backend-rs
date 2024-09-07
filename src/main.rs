use std::{net::SocketAddr, sync::Arc};

use axum::{handler::Handler, routing::get, Extension, Router};
use maa_backend::{
    init_logger,
    middleware::{access_limit::AccessLimitLayer, cors_middleware},
    route::ark_level_handler::get_levels,
    AppState,
};

#[tokio::main]
async fn main() {
    let _guard = init_logger();

    #[allow(clippy::expect_used)]
    let app_state = AppState::new().await.expect("Failed to create app state");

    let app_state = Arc::new(app_state);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/arknights/level",
            get(get_levels.layer(AccessLimitLayer::new(10, 60))),
        )
        .layer(cors_middleware())
        // for getting app state in middleware
        .layer(Extension(Arc::clone(&app_state)))
        .with_state(Arc::clone(&app_state))
        // for getting remote address
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
