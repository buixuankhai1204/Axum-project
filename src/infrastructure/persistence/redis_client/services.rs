use crate::core::error::AppResult;
use crate::infrastructure::persistence::redis_client::instance::{RedisClient, RedisClientExt};
use crate::util::constant::{EXPIRE_FORGET_PASS_CODE_SECS, EXPIRE_SESSION_CODE_SECS};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;
use uuid::Uuid;

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
    pub user_id: Uuid,
}

impl RedisKey for SessionKey {
    type Value = Uuid;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY_{}", self.user_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ForgetPasswordKey {
    pub user_id: Uuid,
}

impl RedisKey for ForgetPasswordKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_FORGET_PASS_CODE_SECS;
}

impl Display for ForgetPasswordKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FORGET_PASS_KEY_{}", self.user_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginValue {
    pub code: String,
}

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
where
    K: RedisKey,
{
    tracing::info!("Set value to redis_client key :{key:?} value :{value:?}");
    let value = serde_json::to_string(value)?;
    client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
    Ok(())
}

pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
where
    K: RedisKey,
{
    tracing::info!("Get value from redis_client key :{key}");
    Ok(client
        .get(&key.to_string())
        .await?
        .map(|v| serde_json::from_str::<K::Value>(&v))
        .transpose()?)
}

pub async fn del(client: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    tracing::info!("Delete key in redis_client :{key:?}");
    client.del(&key.to_string()).await
}

pub async fn get_tll(client: &RedisClient, key: &impl RedisKey) -> AppResult<i64> {
    tracing::info!("Get ttl key in redis_client :{key:?}");
    client.ttl(&key.to_string()).await
}

pub async fn check_exist_key(redis: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    Ok(redis.exist(&key.to_string()).await?)
}
