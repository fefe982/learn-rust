// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// 121. Best Time to Buy and Sell Stock
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = i32::MAX;
        for &p in prices.iter() {
            if p - min_price > max_profit {
                max_profit = p - min_price;
            }
            if p < min_price {
                min_price = p;
            }
        }
        max_profit
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
