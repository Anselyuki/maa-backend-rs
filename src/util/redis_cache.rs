use bb8::Pool;
use redis::{
    AsyncCommands, ExistenceCheck, FromRedisValue, SetExpiry, SetOptions,
    ToRedisArgs,
};

use crate::{
    repository::redis_connection_manager::RedisConnectionManager, MaaResult,
};

pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
}

impl RedisCache {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }

    pub async fn get<T: FromRedisValue + Send + Sync>(
        &self,
        key: &str,
    ) -> MaaResult<Option<T>> {
        let mut conn = self.pool.get().await?;
        let value: Option<T> = conn.get(key).await?;
        Ok(value)
    }

    pub async fn set<T: ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> MaaResult<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set(key, value).await?;
        Ok(())
    }

    pub async fn set_ex<T: ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
        seconds: u64,
    ) -> MaaResult<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set_ex(key, value, seconds).await?;
        Ok(())
    }

    pub async fn set_if_not_exists<T: ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> MaaResult<bool> {
        let mut conn = self.pool.get().await?;
        let result: bool = conn.set_nx(key, value).await?;
        Ok(result)
    }

    pub async fn set_if_not_exists_ex<T: ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
        seconds: u64,
    ) -> MaaResult<bool> {
        let mut conn = self.pool.get().await?;
        let options = SetOptions::default()
            .conditional_set(ExistenceCheck::NX)
            .with_expiration(SetExpiry::EX(seconds));
        let result: bool = conn.set_options(key, value, options).await?;
        Ok(result)
    }

    pub async fn delete_if_equals<
        T: ToRedisArgs + FromRedisValue + Send + Sync + PartialEq,
    >(
        &self,
        key: &str,
        value: T,
    ) -> MaaResult<bool> {
        let mut conn = self.pool.get().await?;
        let saved: Option<T> = conn.get(key).await?;
        if let Some(saved) = saved {
            if saved == value {
                let _: () = conn.del(key).await?;
                return Ok(true);
            }
        }
        Ok(false)
    }
}
