use redis::{AsyncTypedCommands, Client, RedisResult, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::OnceCell;
static SHARED_PUB_SUB_CONNECTION_CELL: OnceCell<Arc<MultiplexedConnection>> = OnceCell::const_new();
static SHARED_PUBLISHER_CONNECTION_CELL: OnceCell<Arc<MultiplexedConnection>> =
    OnceCell::const_new();

pub struct RedisManager {
    pub_sub_connection: Arc<MultiplexedConnection>,
    publisher_connection: Arc<MultiplexedConnection>,
}

impl RedisManager {
    async fn new_pubsub_connection() -> RedisResult<Arc<MultiplexedConnection>> {
        let client = Client::open("redis://127.0.0.1:6379").await?;
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(Arc::new(conn))
    }

    async fn new_publisher_connection() -> RedisResult<Arc<MultiplexedConnection>> {
        let client = Client::open("redis://127.0.0.1:6379").await?;
        let conn = clinet.get_muliplexed_async_connection().await?;
        Ok(Arc::new(conn))
    }

    async fn get_instance() -> RedisResult<RedisManager> {
        static MANAGER_INSTANCE_CELL: OnceCell<RedisManager> = OnceCell::const_new();
        let instance = MANAGER_INSTANCE_CELL
            .get_or_try_init(|| async {
                let pubsub_conn = SHARED_PUB_SUB_CONNECTION_CELL
                    .get_or_try_init(Self::new_pubsub_connection)
                    .await?;

                let publisher_conn = SHARED_PUBLISHER_CONNECTION_CELL
                    .get_or_try_init(Self::new_publisher_connecion)
                    .await?;

                Ok(RedisManager {
                    pub_sub_connection: pubsub_conn.clone(),
                    publisher_connection: publisher_conn.clone(),
                })
            })
            .await?;
        Ok(instance.clone())
    }
}
