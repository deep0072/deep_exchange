use actix_web::dev::always_ready;
use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
mod redis_manager;
mod trade;
use redis_manager::RedisManager;
use std::time::Duration;
use tokio::time::sleep;
use trade::engine::Engine;

use crate::types::api_message::MessageToEngine;
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
struct OuterMessage {
    id: String,
    message: MessageToEngine,
}

#[actix_web::main]
async fn main() -> RedisResult<()> {
    println!("Hello, world!");
    let mut engine = Engine { orderbooks: vec![] };

    let redis_conn = RedisManager::get_instance().await?;

    loop {
        let response: Option<OuterMessage> = redis_conn.pop_message("order").await?;

        let new_response = match response {
            Some(msg) => msg,
            None => {
                eprintln!("Redis connection error:  Retrying in 5 seconds...",);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        };
        match engine.process_order(new_response.message).await {
            Ok((filled_qty, executed_qty, user_id)) => {
                redis_conn
                    .send_to_api(&user_id.to_string(), "recieved order call back".to_string())
                    .await;
            }

            Err(e) => {
                eprintln!("Error processing order: {}. Retrying in 5 seconds...", e);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
        // match new_response {
        //     msg => {
        //         let engine_response = engine.process_order(msg.message).await;
        //         let (filled, executed, user_id) = engine_response.unwrap();

        //         redis_conn
        //             .send_to_api(&user_id.to_string(), "recieved".to_string())
        //             .await;
        //     }
        // }
    }
}
