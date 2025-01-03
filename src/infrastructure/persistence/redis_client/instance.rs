use crate::core::configure::AppConfig;
use crate::core::error::AppResult;
use redis::{Client, RedisError};
use std::time::Duration;

pub type RedisClient = Client;

pub trait RedisClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> AppResult<Self>;
}

pub trait RedisClientExt: RedisClientBuilder {
    fn ping(&self) -> impl std::future::Future<Output = AppResult<Option<String>>>;
    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl std::future::Future<Output = AppResult<()>>;
    fn exist(&self, key: &str) -> impl std::future::Future<Output = AppResult<bool>>;
    fn get(&self, key: &str) -> impl std::future::Future<Output = AppResult<Option<String>>>;
    fn del(&self, key: &str) -> impl std::future::Future<Output = AppResult<bool>>;
    fn ttl(&self, key: &str) -> impl std::future::Future<Output = AppResult<i64>>;
}

impl RedisClientBuilder for RedisClient {
    fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        Ok(Client::open(config.redis.get_url())?)
    }
}

impl RedisClientExt for Client {
    async fn ping(&self) -> AppResult<Option<String>> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
        tracing::info!("ping redis_client server");
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult<()> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg: String = redis::cmd("SET").arg(&[key, value]).query_async(&mut conn).await?;
        tracing::info!("set key redis_client: {msg}");
        let msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await?;
        tracing::info!("set expire time redis_client: {msg}");
        Ok(())
    }

    async fn exist(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        tracing::info!("check key exists: {key}");
        Ok(value)
    }

    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        tracing::info!("get value: {key}");
        Ok(value)
    }

    async fn del(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        tracing::info!("delete value: {key}");
        Ok(value == 1)
    }

    async fn ttl(&self, key: &str) -> AppResult<i64> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        tracing::info!("get TTL value: {key}");
        Ok(value)
    }
}
