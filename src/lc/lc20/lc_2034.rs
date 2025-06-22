// https://leetcode.com/problems/stock-price-fluctuation/
// 2034. Stock Price Fluctuation
pub struct StockPrice {
    timestamp: i32,
    price: i32,
    prices: std::collections::HashMap<i32, i32>,
    maxh: std::collections::BinaryHeap<(i32, i32)>,
    minh: std::collections::BinaryHeap<(std::cmp::Reverse<i32>, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            price: 0,
            prices: std::collections::HashMap::new(),
            maxh: std::collections::BinaryHeap::new(),
            minh: std::collections::BinaryHeap::new(),
        }
    }

    pub fn update(&mut self, timestamp: i32, price: i32) {
        if timestamp >= self.timestamp {
            self.timestamp = timestamp;
            self.price = price;
        }
        self.prices.insert(timestamp, price);
        self.maxh.push((price, timestamp));
        self.minh.push((std::cmp::Reverse(price), timestamp));
    }

    pub fn current(&self) -> i32 {
        self.price
    }

    pub fn maximum(&mut self) -> i32 {
        while let Some(&(p, t)) = self.maxh.peek() {
            if *self.prices.get(&t).unwrap() == p {
                return p;
            }
            self.maxh.pop();
        }
        -1
    }

    pub fn minimum(&mut self) -> i32 {
        while let Some(&(p, t)) = self.minh.peek() {
            if *self.prices.get(&t).unwrap() == p.0 {
                return p.0;
            }
            self.minh.pop();
        }
        -1
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
#[cfg(test)]
mod tests {
    use super::*;
    enum Call {
        Update(i32, i32),
        Current(i32),
        Maximum(i32),
        Minimum(i32),
    }
    #[test]
    fn test() {
        let cases = vec![vec![
            Call::Update(1, 10),
            Call::Update(2, 5),
            Call::Current(5),
            Call::Maximum(10),
            Call::Update(1, 3),
            Call::Maximum(5),
            Call::Update(4, 2),
            Call::Minimum(2),
        ]];
        for case in cases {
            let mut obj = StockPrice::new();
            for call in case {
                match call {
                    Call::Update(timestamp, price) => obj.update(timestamp, price),
                    Call::Current(price) => assert_eq!(obj.current(), price),
                    Call::Maximum(price) => assert_eq!(obj.maximum(), price),
                    Call::Minimum(price) => assert_eq!(obj.minimum(), price),
                }
            }
        }
    }
}
