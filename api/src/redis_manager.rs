use crate::routes::types::GetDepthData;
use crate::routes::{orders::order, types::MessageToEngine};
use rand::{Rng, rng};
use redis::{AsyncTypedCommands, Client, RedisResult, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize, ser};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::OnceCell;
use tokio::time::{Duration, timeout};
use tokio_stream::StreamExt; // Required to get messages from the stream

static SHARED_PUBLISHER_CONNECTION_CELL: OnceCell<Arc<MultiplexedConnection>> =
    OnceCell::const_new();
static CONNECTION_ATTEMPT_COUNT: AtomicUsize = AtomicUsize::new(0);
#[derive(Debug, Clone)]
pub struct RedisManager {
    publisher_connection: Arc<MultiplexedConnection>,
}

impl RedisManager {
    async fn new_publisher_connection() -> RedisResult<Arc<MultiplexedConnection>> {
        let client = Client::open("redis://127.0.0.1:6379")?;
        let count = CONNECTION_ATTEMPT_COUNT.fetch_add(1, Ordering::SeqCst);
        println!(
            "Attempting to create a new Redis connection. Attempt number: {}",
            count + 1
        );

        let conn = client.get_multiplexed_async_connection().await?;

        Ok(Arc::new(conn))
    }

    pub async fn get_instance() -> RedisResult<RedisManager> {
        static MANAGER_INSTANCE_CELL: OnceCell<RedisManager> = OnceCell::const_new();

        let instance = MANAGER_INSTANCE_CELL
            .get_or_try_init(|| async {
                let publisher_conn = SHARED_PUBLISHER_CONNECTION_CELL
                    .get_or_try_init(|| RedisManager::new_publisher_connection())
                    .await?;

                Ok::<RedisManager, redis::RedisError>(RedisManager {
                    publisher_connection: publisher_conn.clone(),
                })
            })
            .await?;
        Ok(instance.clone())
    }

    pub async fn send_and_await(&self, message: MessageToEngine) -> RedisResult<()> {
        let sub_client = Client::open("redis://127.0.0.1:6379")?;
        let mut pubsub = sub_client.get_async_pubsub().await?;

        println!("sending message {:?}", message);

        let user_id = match &message {
            MessageToEngine::CreateOrder(msg) => msg.user_id.unwrap_or(0),
            MessageToEngine::GetDepth(_) => 0,
        };

        let id = Self::generate_time_based_random_id(user_id);
        pubsub.subscribe(&id).await?;
        let serialized_data = serde_json::to_string(&serde_json::json!({
            "id": id,
            "message": message
        }))
        .map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Serialization error",
                e.to_string(),
            ))
        })?;

        let mut conn = (*self.publisher_connection).clone();
        conn.lpush("order", serialized_data).await?;

        // Create message stream and wait for response
        let mut pubsub_stream = pubsub.on_message();

        let timeout_duration = Duration::from_secs(10);

        match timeout(timeout_duration, pubsub_stream.next()).await {
            Ok(Some(msg)) => {
                println!("Raw message received: {:?}", msg);

                // Get the payload as string
                let payload: String = msg.get_payload()?;
                println!("Message payload: {}", payload);

                // Try to parse as JSON if needed
                match serde_json::from_str::<serde_json::Value>(&payload) {
                    Ok(json_value) => {
                        println!(
                            "Parsed JSON response: {}",
                            serde_json::to_string_pretty(&json_value).unwrap_or_default()
                        );
                    }
                    Err(_) => {
                        println!("Response (plain text): {}", payload);
                    }
                }

                Ok(())
            }
            Ok(None) => {
                println!("Stream ended without receiving a message");
                Err(redis::RedisError::from((
                    redis::ErrorKind::IoError,
                    "Stream ended",
                    "Message stream ended unexpectedly".to_string(),
                )))
            }
            Err(e) => {
                println!("Timeout waiting for response on channel: {} {}", e, id);
                Err(redis::RedisError::from((
                    redis::ErrorKind::IoError,
                    "Timeout",
                    format!(
                        "No response received within {} seconds",
                        timeout_duration.as_secs()
                    ),
                )))
            }
        }
    }

    pub fn generate_time_based_random_id(user_id: u32) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let timestamp = now.as_millis();
        let nanos = now.subsec_nanos();
        let suffix = (nanos ^ user_id) & 0xFFFF;

        format!("{}_{}_{}", timestamp, nanos, suffix)
    }
}
