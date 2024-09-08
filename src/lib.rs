pub mod envs;
pub mod middleware;
pub mod repository;
pub mod route;
pub mod service;
pub mod util;

use std::sync::Arc;
use std::{borrow::Cow, fmt::Write};

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use bb8::Pool;
use envs::{db_uri, log_dir, log_prefix, redis_uri};
use http::StatusCode;
use mongodb::Client;
use repository::{
    ark_level_repository::ArkLevelRepository,
    redis_connection_manager::RedisConnectionManager,
};
use thiserror::Error;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Error, Debug)]
pub enum MaaError {
    /**
     * Internal errors
     */

    #[error("Error getting env var: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Error serializing struct: {0}")]
    SerializeError(#[from] bson::ser::Error),

    #[error("Error doing database operations: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("No default database found")]
    NoDefaultDBError,

    #[error("Error doing redis operations: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Error getting redis connection: {0}")]
    RedisPoolError(#[from] bb8::RunError<redis::RedisError>),

    #[error("Error hashing password: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Error parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Jwt error: {0}")]
    JsonWebTokensError(#[from] jsonwebtokens::error::Error),

    /**
     * Business errors
     */

    #[error("用户不存在或密码错误")]
    LoginFail,

    #[error("用户未启用")]
    UserNotEnabled,

    #[error("JWT验证失败")]
    JwtVerifyFailed,

    #[error("用户id不存在")]
    NoneUserId,

    #[error("Validate失败: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for MaaError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match &self {
            MaaError::LoginFail => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::UserNotEnabled => Response::builder()
                .status(10003)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::NoneUserId => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::ValidationError(errors) => {
                let field_errors = errors.field_errors();
                let mut error_msg = String::new();
                for (field, errors) in field_errors {
                    for error in errors {
                        let message = match error.message {
                            Some(ref msg) => match msg {
                                Cow::Borrowed(msg) => msg.to_owned(),
                                Cow::Owned(msg) => msg,
                            },
                            None => "Validation failed",
                        };
                        if let Err(e) =
                            writeln!(error_msg, "{}: {}", field, message)
                        {
                            tracing::error!(
                                "Error writing error message: {}",
                                e
                            );
                        }
                    }
                }
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(error_msg.into())
                    .unwrap_or_default()
            }
            _ => {
                tracing::error!("{}", self);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

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
    pub redis_pool: Pool<RedisConnectionManager>,
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

        Ok(Self {
            ark_level_repository,
            redis_pool,
        })
    }
}
