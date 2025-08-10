use actix_web::http::header::EXPECT;
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::trade::order_book::{OrderType, filled, order, order_book};
use crate::types::api_message::MessageToEngine;

pub struct Engine {
    pub orderbooks: Vec<order_book>,
    // pub  balance: HashMap<&'a str, u32>,
}

impl Engine {
    async fn create_order(
        &mut self,
        market: String,
        price: String,
        quantity: String,
        user_id: String,
        side: String,
    ) -> Result<(Vec<filled>, u32, u32), String> {
        let filled_qty: Vec<filled>;
        let executed_qty: u32;
        let order_type: OrderType;
        if side == "BUY" {
            order_type = OrderType::BID
        } else {
            order_type = OrderType::ASK
        }

        let order_id = self.generate_order_id(&user_id);

        // create order struct
        let order_payload: order = order {
            price: price.parse::<u32>().unwrap(),
            quantity: quantity.parse::<u32>().unwrap(),
            user_id: user_id.clone(),
            filled_qty: 0,
            order_id: order_id,
            order_type: order_type.clone(),
        };

        // println!("order book is before execution{:?}", &self.orderbooks);
        let base_asset_quote: Vec<&str> = market.split('/').collect();

        let mut maybe_book = self
            .orderbooks
            .iter()
            .position((|ob| ob.ticker() == market));

        match maybe_book {
            Some(idx) => {
                let (filled, executed) = self.orderbooks[idx].add_order(order_payload);
                println!("order book is after  execution{:?}", &self.orderbooks);

                Ok((filled, executed, user_id.parse::<u32>().unwrap()))
            }

            None => {
                let mut new_order: order = order {
                    price: price.parse::<u32>().unwrap(),
                    quantity: quantity.parse::<u32>().unwrap(),
                    user_id: user_id,
                    filled_qty: 0,
                    order_id: order_id,
                    order_type: order_type,
                };

                let mut orderbook: order_book = order_book {
                    bids: vec![new_order],
                    asks: Vec::new(),
                    base_asset: base_asset_quote[0].to_string(),
                    quote_asset: base_asset_quote[1].to_string(),
                    last_traded_id: 0,
                    current_price: 0,
                };
                let depth: &(Vec<(u32, u32)>, Vec<(u32, u32)>) = &orderbook.get_depth();
                println!("order book depth is after  execution{:?}", depth);

                self.orderbooks.push(orderbook);

                Err("Market not found".to_string())
            }
        }
    }

    pub async fn process_order(
        &mut self,
        message: MessageToEngine,
    ) -> Result<(Vec<filled>, u32, u32), String> {
        match message {
            MessageToEngine::CreateOrder(msg) => {
                let user_id = msg.user_id.unwrap().to_string();

                Ok(self
                    .create_order(
                        msg.market,
                        msg.price.to_string(),
                        msg.quantity.to_string(),
                        user_id,
                        msg.side,
                    )
                    .await?)
            }

            MessageToEngine::GetDepth(msg) => Ok((Vec::<filled>::new(), 0, 0)),
        }
    }

    fn generate_order_id(&mut self, user_id: &str) -> u32 {
        use std::hash::{Hash, Hasher};
        let now: DateTime<Utc> = Utc::now();
        let timestamp = now.timestamp();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (user_id, timestamp).hash(&mut hasher);
        let order_id = hasher.finish() as u32;
        order_id
    }
}
