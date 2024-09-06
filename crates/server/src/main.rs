use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use maa_backend::{
    envs::{db_uri, log_dir, log_prefix},
    repository::ark_level_repository::ArkLevelRepository,
    MaaError, MaaResult,
};
use mongodb::Client;
use tracing_appender::non_blocking::WorkerGuard;

pub struct AppState {
    pub ark_level_repository: ArkLevelRepository,
}

pub type MaaAppState = State<Arc<AppState>>;

#[tokio::main]
async fn main() {
    let _guard = init_logger();

    #[allow(clippy::expect_used)]
    let app_state = AppState::new().await.expect("Failed to create app state");

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .with_state(Arc::new(app_state));

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

impl AppState {
    async fn new() -> MaaResult<Self> {
        let uri = db_uri()?;
        let client = Client::with_uri_str(&uri).await?;
        let db = client
            .default_database()
            .ok_or(MaaError::NoDefaultDBError)?;

        let ark_level_repository = ArkLevelRepository::new(&db);

        Ok(Self {
            ark_level_repository,
        })
    }
}
