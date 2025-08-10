use serde::{Deserialize, Serialize};
use serde_json;

// pub order_type: String,
// pub market_pair: String,
// pub price: u32,
// pub quantity: u32,
// pub user_id: Option<u32>,
// pub side: String,

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum MessageToEngine {
    CreateOrder(CreateOrderData),
    GetDepth(GetDepthData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderData {
    pub market: String,
    pub price: u32,
    pub quantity: u32,
    pub user_id: Option<u32>,
    pub side: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDepthData {
    pub market_pair: String,
}
