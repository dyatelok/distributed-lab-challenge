#![feature(let_chains)]
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, ops::Neg};
use serde::Deserialize;
use std::io::{BufRead, Write};

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Seller,
    Buyer,
}

#[derive(Clone, Copy, Debug)]
pub struct Trade {
    user_id: u64,
    amount: u64,
    // represents how much of other asset person gives for this asset
    // In reality this'd be some kind of fraction, but here it's integer for the sake of simplicity
    // Price - price for 1 base in quote. The total price will be the amount * price.
    price: u64,
}

impl Trade {
    pub fn from(user_id: u64, amount: u64, price: u64) -> Self {
        Self {
            user_id,
            amount,
            price,
        }
    }
}

impl Ord for Trade {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for Trade {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Trade {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Trade {}

#[derive(Clone, Copy, Debug)]
pub enum Currency {
    UAH,
    USD,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::UAH => "UAH",
            Self::USD => "USD",
        };

        write!(f, "{}", str)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BalanceChange {
    pub user_id: u64,
    pub value: i64,
    pub currency: Currency,
}

impl BalanceChange {
    fn from(user_id: u64, value: i64, currency: Currency) -> Self {
        Self {
            user_id,
            value,
            currency,
        }
    }
}

pub struct OrderBook {
    sells: BinaryHeap<Reverse<Trade>>,
    buys: BinaryHeap<Trade>,
    base: Currency,
    quote: Currency,
}

impl OrderBook {
    #[allow(clippy::new_without_default)]
    pub fn new(base: Currency, quote: Currency) -> Self {
        Self {
            sells: Default::default(),
            buys: Default::default(),
            base,
            quote,
        }
    }

    pub fn new_order(&mut self, trade: Trade, side: Side) -> Vec<BalanceChange> {
        match side {
            Side::Seller => self.sells.push(Reverse(trade)),
            Side::Buyer => self.buys.push(trade),
        }

        // here we assume that trade requests are all valid. In reality trades should be wrapped in type hat can only be constructed after balance validation
        self.match_trades()
    }

    fn match_trades(&mut self) -> Vec<BalanceChange> {
        let mut balance_changes = vec![];

        // There could be a better matching algorithm, but it stays in problem
        // The top of the book is where you'll find the highest bid and lowest ask prices. So when we have a price match top sell price <= top buy price we have a match So the person from the sell-side automatically sells UAH(base) and from the buy-side automatically buys UAH(base).

        loop {
            let (Some(mut top_seller), Some(mut top_buyer)) =
                (self.sells.peek_mut(), self.buys.peek_mut())
            else {
                break;
            };

            if top_seller.0.price <= top_buyer.price {
                // amount to transfer is the smallest of both
                let amount = top_seller.0.amount.min(top_buyer.amount);

                // is's totally valid to panic in this situatuon since it'll result in huge problems in case of this passing. This error type could be prevented using wrappers that insure that numbers <= i64::MAX. It's omitted for simplicity
                // price is determined by buyer (always bigger than seller's so no need to take max)
                let price = i64::try_from(top_buyer.price)
                    .expect("Could not convert price from unsigned to signed");
                let amount_uah = i64::try_from(amount)
                    .expect("Could not convert transaction from unsigned to signed");
                // is's totally valid to panic in this situatuon since it'll result in problems in case of this passing, also recoverable here, in real world HFT this could be done using number types that can't overflow
                let amount_usd = amount_uah
                    .checked_mul(price)
                    .expect("Attempt to multiply by price with overflow");

                // buyer  -- UAH -> seller
                balance_changes.push(BalanceChange::from(
                    top_buyer.user_id,
                    amount_uah.neg(),
                    self.base,
                ));
                balance_changes.push(BalanceChange::from(
                    top_seller.0.user_id,
                    amount_uah,
                    self.base,
                ));
                // seller -- USD -> buyer
                balance_changes.push(BalanceChange::from(
                    top_seller.0.user_id,
                    amount_usd.neg(),
                    self.quote,
                ));
                balance_changes.push(BalanceChange::from(
                    top_buyer.user_id,
                    amount_usd,
                    self.quote,
                ));

                // change proposals, valid because amount is the smaller of two
                top_seller.0.amount -= amount;
                top_buyer.amount -= amount;

                let top_seller_amount = top_seller.0.amount;
                let top_buyer_amount = top_buyer.amount;

                drop(top_seller);
                drop(top_buyer);

                // try to remove orders from the book
                if top_seller_amount == 0 {
                    self.sells.pop();
                }
                if top_buyer_amount == 0 {
                    self.buys.pop();
                }
            } else {
                break;
            }
        }

        balance_changes
    }
}

#[derive(Deserialize, Debug)]
struct Order {
    user_id: u64,
    amount: u64,
    price: u64,
    is_seller_side: bool,
}

fn try_parse_order(input: &str) -> Option<(Trade, Side)> {
    ron::from_str::<Order>(input).ok().map(|order| {
        (
            Trade::from(order.user_id, order.amount, order.price),
            match order.is_seller_side {
                true => Side::Seller,
                false => Side::Buyer,
            },
        )
    })
}

pub fn generic_main<R: BufRead, W: Write>(mut input: R, mut output: W) {
        let mut orderbook = OrderBook::new(Currency::UAH, Currency::USD);

    let mut buff = String::new();

    loop {
        buff.truncate(0);
        let _ = input.read_line(&mut buff).expect("Failed to read line");
        if buff.is_empty() {
            break;
        }

        let (order, side) = try_parse_order(buff.as_str()).expect("Failed to parse object");
        orderbook
                .new_order(order, side)
                .into_iter()
                .for_each(|balance_change| {
                    let user_id = balance_change.user_id;
                    let value = balance_change.value;
                    let currency = balance_change.currency;
                    
                    output
                        .write_all(format!("BalanceChange{{user_id: {user_id:5}, value: {value:5}, currency: \"{currency}\"}}\n").as_bytes())
                        .expect("Failed to write to out")
                });
        output.flush().expect("Failed to flush out");
    }

    // output.write_all(b"Finished executing, no more orders").expect("Failed to write to out");
    // output.flush().expect("Failed to flush out");

}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, };
   
    use super::*;

    #[test]
    fn test_orderbook() {
        let input = b"Order(user_id: 1, amount: 10, price: 25, is_seller_side: true)
Order(user_id: 5, amount: 10, price: 35, is_seller_side: true)
Order(user_id: 2, amount: 50, price: 30, is_seller_side: false)
Order(user_id: 3, amount: 10, price: 32, is_seller_side: false)
Order(user_id: 7, amount: 10, price: 35, is_seller_side: false)
Order(user_id: 4, amount: 5, price: 3, is_seller_side: false)
Order(user_id: 6, amount: 100, price: 15, is_seller_side: true)";
        let c = Cursor::new(input);
        let mut buff_reader= BufReader::new(c);

        let mut write_cursor = Cursor::new(Vec::<u8>::new());

        generic_main(&mut buff_reader,&mut write_cursor);
        
        assert_eq!(write_cursor.into_inner(), br###"BalanceChange{user_id:     2, value:   -10, currency: "UAH"}
BalanceChange{user_id:     1, value:    10, currency: "UAH"}
BalanceChange{user_id:     1, value:  -300, currency: "USD"}
BalanceChange{user_id:     2, value:   300, currency: "USD"}
BalanceChange{user_id:     7, value:   -10, currency: "UAH"}
BalanceChange{user_id:     5, value:    10, currency: "UAH"}
BalanceChange{user_id:     5, value:  -350, currency: "USD"}
BalanceChange{user_id:     7, value:   350, currency: "USD"}
BalanceChange{user_id:     3, value:   -10, currency: "UAH"}
BalanceChange{user_id:     6, value:    10, currency: "UAH"}
BalanceChange{user_id:     6, value:  -320, currency: "USD"}
BalanceChange{user_id:     3, value:   320, currency: "USD"}
BalanceChange{user_id:     2, value:   -40, currency: "UAH"}
BalanceChange{user_id:     6, value:    40, currency: "UAH"}
BalanceChange{user_id:     6, value: -1200, currency: "USD"}
BalanceChange{user_id:     2, value:  1200, currency: "USD"}
"###)
    }
}
