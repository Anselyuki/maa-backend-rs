use axum::async_trait;
use bb8::ManageConnection;
use redis::Client;

pub struct RedisConnectionManager {
    redis_client: Client,
}

#[async_trait]
impl ManageConnection for RedisConnectionManager {
    type Connection = redis::aio::MultiplexedConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.redis_client.get_multiplexed_tokio_connection().await
    }

    async fn is_valid(
        &self,
        conn: &mut Self::Connection,
    ) -> Result<(), Self::Error> {
        conn.send_packed_command(&redis::cmd("PING")).await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

impl RedisConnectionManager {
    pub fn new(redis_client: Client) -> Self {
        Self { redis_client }
    }
}
