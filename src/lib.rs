pub mod envs;
pub mod repository;
pub mod route;

use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use envs::{db_uri, log_dir, log_prefix};
use mongodb::Client;
use repository::ark_level_repository::ArkLevelRepository;
use thiserror::Error;
use tracing_appender::non_blocking::WorkerGuard;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Error, Debug)]
pub enum MaaError {
    #[error("Error getting env var: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Error serializing struct: {0}")]
    SerializeError(#[from] bson::ser::Error),

    #[error("Error doing database operations: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("No default database found")]
    NoDefaultDBError,
}

impl IntoResponse for MaaError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

// We expect these env vars to be set at runtime so we can call expect here
#[allow(clippy::expect_used)]
pub fn init_logger() -> WorkerGuard {
    let log_dir = log_dir().expect("LOG_DIR is not set");
    let log_prefix = log_prefix().expect("LOG_PREFIX is not set");

    let log_writer = tracing_appender::rolling::daily(log_dir, log_prefix);
    let (appender, guard) = tracing_appender::non_blocking(log_writer);

    tracing_subscriber::fmt().with_writer(appender).init();

    guard
}

pub struct AppState {
    pub ark_level_repository: ArkLevelRepository,
}

pub type MaaAppState = State<Arc<AppState>>;

impl AppState {
    pub async fn new() -> MaaResult<Self> {
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
