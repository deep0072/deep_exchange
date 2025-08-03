use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPayload {
    pub order_type: String,
    pub market_pair: String,
    pub price: u32,
    pub quantity: u32,
    pub user_id: Option<u32>,
    pub side: String,
}
