enum OrderType {
    ASK,
    BID,
}

struct order<'a> {
    price: u32,
    quantity: u32,
    filled_qty: u32,
    order_id: u32,
    order_type: OrderType,
    user_id: &'a str,
}

struct filled<'a> {
    price: u32,
    quantity: u32,
    trade_id: u32,
    other_user_id: &'a str,
    market_order_id: u32,
}

struct order_book<'a> {
    asks: Vec<order<'a>>,
    bids: Vec<order<'a>>,
    base_asset: &'a str,
    quote_asset: &'a str,
    last_traded_id: u32,
    current_price: u32,
}

impl<'a> order_book<'a> {
    fn new(
        asks: Vec<order<'a>>,
        bids: Vec<order<'a>>,
        base_asset: &'a str,
        quote_asset: &'a str,
        last_traded_id: u32,
        current_price: u32,
    ) -> Self {
        Self {
            asks,
            bids,
            base_asset,
            quote_asset,
            last_traded_id,
            current_price,
        }
    }

    fn match_ask(&mut self, sell_order: order) ->(Vec<filled>,u32) {
        let mut executed_qty: u32 = 0;
        let mut fills: Vec<filled> = Vec::new();
        for ask in self.asks.iter_mut() {
            if ask.price <= sell_order.price && executed_qty <= sell_order.quantity {
                let filled_qty = (sell_order.quantity - executed_qty).min(ask.quantity);
                executed_qty += filled_qty;
                ask.filled_qty = filled_qty;
                self.last_traded_id += 1;

                fills.push(filled {
                    price: ask.price,
                    quantity: filled_qty,
                    trade_id: self.last_traded_id,
                    other_user_id: ask.user_id,
                    market_order_id: ask.order_id,
                });
            }
        }

        // filter out the filled quanity from asks list
        self.asks.retain(|ask| ask.filled_qty!=ask.quantity);
        return (fills, executed_qty);

    fn add_order(&mut self, placed_order: order) {
        if placed_order.order_type == OrderType::ASK {
            // match_ask
        } else {
        }
    }
}
