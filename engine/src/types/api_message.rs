pub struct message_from_api {
    pub market_pair: String,
    pub price: String,
    pub quantity: String,
    pub user_id: Option<u32>,
    pub side: String,
}
