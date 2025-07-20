use redis::{AsyncTypedCommands, Client, RedisResult, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::OnceCell;
static SHARED_CONNECTION_CELL: OnceCell<Arc<MultiplexedConnection>> = OnceCell::const_new();

pub struct RedisManager {
    connection: Arc<MultiplexConnection>,
}

impl RedisManager {
    async fn new_connection() -> RedisResult<Arc<MultiplexConnection>> {
        let redis_url = "redis://localhost:6379";
        let client = Client::open(redis_url)?;
    }
}
