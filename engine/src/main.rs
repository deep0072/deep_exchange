use actix_web::dev::always_ready;
use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
mod redis_manager;
mod trade;
use redis_manager::RedisManager;
use std::time::Duration;
use tokio::time::sleep;
use trade::engine::Engine;

use crate::types::api_message::message_from_api;
mod types;

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
    let mut engine = Engine { orderbooks: vec![] };

    let redis_conn = RedisManager::get_instance().await?;

    loop {
        let response: RedisResult<Option<message>> = redis_conn.pop_message("order").await;
        match response {
            Ok(Some(res)) => {
                println!("popped message: {:?}", res);
                let user_id = res.id;
                let order_msg: message_from_api = message_from_api {
                    market_pair: res.message.market_pair,
                    price: res.message.price.to_string(),
                    quantity: res.message.quantity.to_string(),
                    user_id: res.message.user_id,
                    side: res.message.side,
                };
                engine.process_order(order_msg).await;

                redis_conn
                    .send_to_api(&user_id, "reciedved".to_string())
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
