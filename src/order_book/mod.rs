use std::collections::BTreeMap;

pub mod limit;
pub mod side;

use side::Side;

pub struct OrderBook {
    pub sells: BTreeMap<u64, u32>,
    pub buys: BTreeMap<u64, u32>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            sells: BTreeMap::new(),
            buys: BTreeMap::new(),
        }
    }

    /// Return the lowest sell price, if present.
    pub fn min_sell(&self) -> Option<u64> {
        match self.sells.iter().next() {
            Some((price, _)) => Some(*price),
            _ => None,
        }
    }

    /// Return the highest buy price, if present.
    pub fn max_buy(&self) -> Option<u64> {
        match self.buys.iter().rev().next() {
            Some((price, _)) => Some(*price),
            _ => None,
        }
    }

    /// Return the difference of the lowest ask and highest bid, if both are
    /// present.
    pub fn spread(&self) -> Option<u64> {
        match (self.max_buy(), self.min_sell()) {
            (Some(b), Some(a)) => Some((a as i64 - b as i64).abs() as u64),
            _ => None,
        }
    }

    pub fn add_limit(
        &mut self,
        side: Side,
        size: u32,
        price: u64,
    ) {
        match side {
            Side::Buy => {
                match size {
                    0 => {
                        self.buys.remove(&price);
                    },
                    _ => {
                        self.buys
                            .entry(price)
                            .or_insert_with(|| size);
                    },
                };
            }
            Side::Sell => {
                match size {
                    0 => {
                        self.sells.remove(&price);
                    },
                    _ => {
                        self.sells
                            .entry(price)
                            .or_insert_with(|| size);
                    },
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_order_book() {
        let order_book = OrderBook::new();

        assert_eq!(order_book.max_buy(), None);
        assert_eq!(order_book.min_sell(), None);
        assert_eq!(order_book.spread(), None);
    }

    #[test]
    fn empty_btreemap_iter() {
        let empty_map: BTreeMap<u64, u32> = BTreeMap::new();
        assert_eq!(empty_map.iter().next(), None);
    }

    #[test]
    fn one_buy() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Buy, 1, 1);

        assert_eq!(order_book.spread(), None);
    }

    #[test]
    fn one_sell() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Sell, 1, 1);

        assert_eq!(order_book.spread(), None);
    }

    #[test]
    fn no_spread() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Sell, 1, 1);
        order_book.add_limit(Side::Buy, 1, 1);

        assert_eq!(order_book.spread().unwrap(), 0);
    }

    #[test]
    fn spread() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Sell, 1, 1);
        order_book.add_limit(Side::Buy, 1, 2);

        assert_eq!(order_book.spread().unwrap(), 1);
    }

    #[test]
    fn max_buy() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Buy, 1, 1);
        order_book.add_limit(Side::Buy,1, 2);

        assert_eq!(order_book.max_buy().unwrap(), 2);
    }

    #[test]
    fn min_sell() {
        let mut order_book = OrderBook::new();

        order_book.add_limit(Side::Sell, 1, 1);
        order_book.add_limit(Side::Sell, 1, 2);

        assert_eq!(order_book.min_sell().unwrap(), 1);
    }
}


