pub mod envs;
pub mod repository;

use thiserror::Error;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Error, Debug)]
pub enum MaaError {
    #[error("Error getting env var: {0}")]
    EnvError(#[from] std::env::VarError),
}
