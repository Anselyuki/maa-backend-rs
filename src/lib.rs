pub mod envs;
pub mod middleware;
pub mod repository;
pub mod route;
pub mod service;
pub mod util;

use std::sync::Arc;

use axum::extract::State;
use bb8::Pool;
use envs::{db_uri, log_dir, log_prefix, redis_uri};
use error::MaaError;
use mongodb::Client;
use repository::{
    ark_level_repository::ArkLevelRepository,
    redis_connection_manager::RedisConnectionManager,
};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use util::redis_cache::RedisCache;

pub type MaaResult<T> = Result<T, MaaError>;

pub mod error;

// We expect these env vars to be set at runtime so we can call expect here
#[allow(clippy::expect_used)]
pub fn init_logger() -> Option<WorkerGuard> {
    let log_dir = log_dir();
    let log_prefix = log_prefix();

    #[cfg(debug_assertions)]
    let log_level = Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let log_level = Level::INFO;

    match (log_dir, log_prefix) {
        (Ok(dir), Ok(prefix)) => {
            let log_writer = tracing_appender::rolling::daily(dir, prefix);
            let (appender, guard) = tracing_appender::non_blocking(log_writer);
            tracing_subscriber::fmt()
                .with_max_level(log_level)
                .with_writer(appender)
                .with_writer(std::io::stdout)
                .init();
            Some(guard)
        }
        _ => {
            println!("Error getting env vars for logging, using stdout");
            tracing_subscriber::fmt().with_max_level(log_level).init();
            None
        }
    }
}

pub struct AppState {
    pub ark_level_repository: ArkLevelRepository,
    pub redis_cache: RedisCache,
}

pub type MaaAppState = State<Arc<AppState>>;

impl AppState {
    pub async fn new() -> MaaResult<Self> {
        // 初始化mongodb连接
        // mongodb driver中自带connection pool，默认大小为10，如需调整可使用`ClientOptions::builder().max_pool_size()`
        let uri = db_uri()?;
        let client = Client::with_uri_str(&uri).await?;
        let db = client
            .default_database()
            .ok_or(MaaError::NoDefaultDBError)?;

        tracing::info!("Connected to database: {}", db.name());

        let ark_level_repository = ArkLevelRepository::new(&db);

        // 初始化redis连接
        let redis_uri = redis_uri()?;
        let redis_client = redis::Client::open(redis_uri)?;
        let redis_connection_manager =
            RedisConnectionManager::new(redis_client);
        let redis_pool =
            Pool::builder().build(redis_connection_manager).await?;

        let redis_cache = RedisCache::new(redis_pool);

        Ok(Self {
            ark_level_repository,
            redis_cache,
        })
    }
}
