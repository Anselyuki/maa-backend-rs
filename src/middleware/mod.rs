pub mod access_limit;

use std::time::Duration;

use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

#[allow(clippy::expect_used)]
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(
            "http://localhost"
                .parse::<HeaderValue>()
                .expect("Failed to parse origin"),
        )
        .allow_credentials(true)
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .max_age(Duration::from_secs(3600))
}
