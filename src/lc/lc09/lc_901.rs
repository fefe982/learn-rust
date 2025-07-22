// https://leetcode.com/problems/online-stock-span/
// 901. Online Stock Span
pub struct StockSpanner {
    prices: Vec<(i32, i32)>,
    day: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    pub fn new() -> Self {
        Self {
            prices: vec![(i32::MAX, 0)],
            day: 0,
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.day += 1;
        while self.prices.last().unwrap().0 <= price {
            self.prices.pop();
        }
        let span = self.day - self.prices.last().unwrap().1;
        self.prices.push((price, self.day));
        span
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stock_span() {
        let mut obj = StockSpanner::new();
        assert_eq!(obj.next(100), 1);
        assert_eq!(obj.next(80), 1);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(70), 2);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(75), 4);
        assert_eq!(obj.next(85), 6);
    }
}
