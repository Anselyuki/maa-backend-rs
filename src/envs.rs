use crate::MaaResult;

fn get_env(key: &str) -> MaaResult<String> {
    std::env::var(key).map_err(Into::into)
}

/**
 * log 相关
 */

pub fn log_dir() -> MaaResult<String> {
    get_env("LOG_DIR")
}

pub fn log_prefix() -> MaaResult<String> {
    get_env("LOG_PREFIX")
}

/**
 * 数据库相关
 */

pub fn db_uri() -> MaaResult<String> {
    get_env("DB_URI")
}

pub fn redis_uri() -> MaaResult<String> {
    get_env("REDIS_URI")
}
