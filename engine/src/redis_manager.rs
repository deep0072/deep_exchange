use std::sync::Arc;

use redis::{AsyncTypedCommands, Client, RedisResult, aio::MultiplexedConnection};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::OnceCell;
static SHARED_CONNECTION_CELL: OnceCell<Arc<MultiplexedConnection>> = OnceCell::const_new();

#[derive(Clone)]
pub struct RedisManager {
    connection: Arc<MultiplexedConnection>,
}

impl RedisManager {
    async fn new_connection() -> RedisResult<Arc<MultiplexedConnection>> {
        let redis_url = "redis://127.0.0.1:6379/".to_string();
        let client = Client::open(redis_url)?;

        let conn = client.get_multiplexed_async_connection().await?;
        println!("successfully connected to redis");
        Ok(Arc::new(conn))
    }

    async fn get_instance() -> RedisResult<&'static RedisManager> {
        static MANAGER_INSTANCE_CELL: OnceCell<RedisManager> = OnceCell::const_new();

        MANAGER_INSTANCE_CELL
            .get_or_try_init(|| async {
                let shared_conn = SHARED_CONNECTION_CELL
                    .get_or_try_init(Self::new_connection)
                    .await?;
                Ok(RedisManager {
                    connection: shared_conn.clone(),
                })
            })
            .await
    }

    pub async fn push_message(&self, list_key: &str, message: &impl Serialize) -> RedisResult<()> {
        let json_string = serde_json::to_string(message).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "json serialization falied",
                e.to_string(),
            ))
        })?;
        let mut conn = (*self.connection).clone();
        conn.lpush(list_key, json_string).await?;
        Ok(())
    }

    pub async fn pop_message<T: DeserializeOwned>(
        &self,
        list_key: &str,
        message: &mut T,
    ) -> RedisResult<()> {
        let mut conn = (*self.connection).clone();
        let json_string: Option<String> = conn.rpop(list_key, None).await?;
        if let Some(json_string) = json_string {
            let deserialized_message: T = serde_json::from_str(&json_string).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "json deserialization failed",
                    e.to_string(),
                ))
            })?;
            *message = deserialized_message;
        }
        Ok(())
    }
}
