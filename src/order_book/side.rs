use std::fmt;

/// An order book side.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Side;

    #[test]
    fn side_negation() {
        assert_eq!(!Side::Sell, Side::Buy);
        assert_eq!(!Side::Buy, Side::Sell);
    }
}
