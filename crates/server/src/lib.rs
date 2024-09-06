pub mod envs;
pub mod repository;

use thiserror::Error;

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
