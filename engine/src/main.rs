use actix_web::dev::always_ready;
use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
mod redis_manager;
mod trade;
use redis_manager::RedisManager;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub market_pair: String,
    pub price: u32,
    pub quantity: u32,
    pub user_id: Option<u32>,
    pub side: String,
}
// {"id":"1753622092379_379312337_55509","message":{"market_pair":"SOL/USDT","price":180,"quantity":1,"side":"BUY","user_id":4}}
#[derive(Serialize, Deserialize, Debug)]
struct message {
    id: String,
    message: Order,
}

#[actix_web::main]
async fn main() -> RedisResult<()> {
    println!("Hello, world!");

    let redis_conn = RedisManager::get_instance().await?;

    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut client_conn = client.get_connection().unwrap();

    loop {
        let response: RedisResult<Option<message>> = redis_conn.pop_message("order").await;
        match response {
            Ok(Some(res)) => {
                println!("popped message: {:?}", res);
                let user_id = res.message.user_id.unwrap();
                redis_conn
                    .send_to_api("user_id", "reciedved".to_string())
                    .await;
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                eprintln!("Redis connection error: {}. Retrying in 5 seconds...", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
