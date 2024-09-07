pub mod access_limit;

use std::time::Duration;

use http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any)
        .allow_credentials(true)
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600))
}
