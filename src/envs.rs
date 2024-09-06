use crate::MaaResult;

fn get_env(key: &str) -> MaaResult<String> {
    std::env::var(key).map_err(Into::into)
}

/// Log相关
pub fn log_dir() -> MaaResult<String> {
    get_env("LOG_DIR")
}

pub fn log_prefix() -> MaaResult<String> {
    get_env("LOG_PREFIX")
}
