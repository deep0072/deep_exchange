use actix_web::http::header::EXPECT;
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::trade::order_book::{OrderType, filled, order, order_book};
use crate::types::api_message::message_from_api;

pub struct Engine<'a> {
    pub orderbooks: Vec<order_book<'a>>,
    // pub  balance: HashMap<&'a str, u32>,
}

impl Engine<'_> {
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
        if side == "buy" {
            order_type = OrderType::BID
        } else {
            order_type = OrderType::ASK
        }

        let order_id = self.generate_order_id(&user_id);

        // create order struct
        let order_payload: order = order {
            price: price.parse::<u32>().unwrap(),
            quantity: quantity.parse::<u32>().unwrap(),
            user_id: &user_id,
            filled_qty: 0,
            order_id: order_id,
            order_type: order_type,
        };

        let maybe_book = self.orderbooks.iter_mut().find(|ob| ob.ticker() == market);
        match maybe_book {
            Some(book) => {
                let (filled, executed) = book.add_order(order_payload);
                Ok((filled, executed, executed))
            }

            None => {
                Err("Market not found".to_string())
                // let mut orderbook:order = order{
                //     price: price.parse::<u32>().unwrap(),
                //     quantity: quantity.parse::<u32>().unwrap(),
                //     user_id: &user_id,
                //     filled_qty: 0,
                //     order_id: order_id,
                //     order_type: order_type,
                // };

                // self.orderbooks.push( order_book.bids.push(order_payload));
                // Ok((0, 0, 0))
            }
        }
    }

    pub async fn process_order(
        &mut self,
        message: message_from_api,
    ) -> Result<(Vec<filled>, u32, u32), String> {
        let user_id = message.user_id.unwrap().to_string();
        self.create_order(
            message.market_pair,
            message.price,
            message.quantity,
            user_id,
            message.side,
        )
        .await
    }

    fn generate_order_id(&mut self, user_id: &str) -> u32 {
        let now: DateTime<Utc> = Utc::now();
        let timestamp = now.to_rfc3339();
        let order_id = format!("{}", user_id);
        println!("{}", &order_id);
        order_id.parse::<u32>().unwrap()
    }
}
