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

// 最大登陆数
pub fn max_login_count() -> MaaResult<usize> {
    get_env("MAX_LOGIN_COUNT")
        .map(|x| x.parse())
        .and_then(|x| x.map_err(Into::into))
}

// JWT key
pub fn jwt_key() -> MaaResult<String> {
    get_env("JWT_KEY")
}

// JWT 过期时间
pub fn jwt_expire_time() -> MaaResult<u64> {
    get_env("JWT_EXPIRE_TIME")
        .map(|x| x.parse())
        .and_then(|x| x.map_err(Into::into))
}
