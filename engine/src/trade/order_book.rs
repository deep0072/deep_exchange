use std::{collections::HashMap, iter, str::SplitAsciiWhitespace};

#[derive(PartialEq, Eq)]
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

    fn match_ask(&mut self, sell_order: order) -> (Vec<filled>, u32) {
        let mut executed_qty: u32 = 0;
        let mut fills: Vec<filled> = Vec::new();
        for bid in self.bids.iter_mut() {
            if bid.price >= sell_order.price && executed_qty <= sell_order.quantity {
                let filled_qty = (sell_order.quantity - executed_qty).min(bid.quantity);
                executed_qty += filled_qty;
                bid.filled_qty = filled_qty;
                self.last_traded_id += 1;
                self.current_price = bid.price;

                fills.push(filled {
                    price: bid.price,
                    quantity: filled_qty,
                    trade_id: self.last_traded_id,
                    other_user_id: bid.user_id,
                    market_order_id: bid.order_id,
                });
            }
        }
        let order_bids_to_remove: Vec<u32> = self
            .bids
            .iter()
            .filter(|bid| bid.filled_qty == bid.quantity)
            .map(|bid| bid.order_id)
            .collect();
        // filter out the filled quanity from asks list
        self.bids
            .retain(|bid| !order_bids_to_remove.contains(&bid.order_id));
        return (fills, executed_qty);
    }

    fn match_bid(&mut self, buy_order: order) -> (Vec<filled>, u32) {
        let mut executed_qty: u32 = 0;
        let mut fills: Vec<filled> = Vec::new();

        for ask in self.asks.iter_mut() {
            if ask.price <= buy_order.price && executed_qty <= buy_order.quantity {
                let fills_qty = (buy_order.quantity - executed_qty).min(ask.quantity);
                executed_qty += fills_qty;
                self.last_traded_id += 1;
                self.current_price = ask.price;

                fills.push(filled {
                    price: ask.price,
                    quantity: executed_qty,
                    trade_id: self.last_traded_id,
                    other_user_id: ask.user_id,
                    market_order_id: ask.order_id,
                })
            }
        }

        // collect filled qty from asks order list
        let asks_order_to_remove: Vec<u32> = self
            .asks
            .iter()
            .filter(|ask| ask.filled_qty == ask.quantity)
            .map(|x| x.order_id)
            .collect();

        self.asks
            .retain(|ask| !asks_order_to_remove.contains(&ask.order_id));

        return (fills, executed_qty);
    }

    fn get_depth(&mut self) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut bids: Vec<(u32, u32)> = Vec::new();
        let mut asks: Vec<(u32, u32)> = Vec::new();
        let mut bids_obj: HashMap<u32, u32> = HashMap::new();
        let mut asks_obj: HashMap<u32, u32> = HashMap::new();

        for ask in self.asks.iter() {
            *asks_obj.entry(ask.price).or_insert(0) += ask.quantity;
        }
        for (key, value) in asks_obj.into_iter() {
            asks.push((key, value));
        }
        for bid in self.bids.iter() {
            *bids_obj.entry(bid.price).or_insert(0) += bid.quantity;
        }

        for (key, value) in bids_obj.into_iter() {
            bids.push((key, value));
        }
        return (bids, asks);
    }

    fn get_open_order(&mut self, user_id: &str) {
        let open_asks: Vec<u32> = self
            .asks
            .iter()
            .filter(|ask| ask.user_id == user_id)
            .map(|ask| ask.order_id)
            .collect();
        let open_order: Vec<u32> = self
            .asks
            .iter()
            .filter(|order| order.user_id == user_id)
            .map(|order| order.order_id)
            .collect();
    }

    fn get_cancel_order(&mut self, user_id: &str, order_type: OrderType) {
        if order_type == OrderType::ASK {
            self.asks.retain(|ask| ask.user_id != user_id);
        } else {
            self.bids.retain(|bid| bid.user_id != user_id);
        }
    }

    fn add_order(&mut self, placed_order: order) -> (Vec<filled>, u32) {
        if placed_order.order_type == OrderType::BID {
            self.match_bid(placed_order)
        } else {
            let (filled_qty, executed_qty) = self.match_ask(placed_order);
            (filled_qty, executed_qty)
        }
    }
}

// fn add_order(&mut self, placed_order: order) {
//     if placed_order.order_type == OrderType::ASK {
//         // match_ask
//     } else {
//         match
//     }
// }
