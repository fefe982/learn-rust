// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/description/
// 123. Best Time to Buy and Sell Stock III
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit_l = vec![0; prices.len()];
        let mut max_profit = 0;
        let mut min_price = i32::MAX;
        for (idx, &p) in prices.iter().enumerate() {
            let profit = std::cmp::max(0, p - min_price);
            max_profit = std::cmp::max(profit, max_profit);
            min_price = std::cmp::min(p, min_price);
            max_profit_l[idx] = max_profit;
        }
        let mut max_price = 0;
        for idx in (1..prices.len()).rev() {
            let p = prices[idx];
            let profit = std::cmp::max(0, max_price - p);
            max_profit = std::cmp::max(profit + max_profit_l[idx - 1], max_profit);
            max_price = std::cmp::max(p, max_price);
        }
        max_profit
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
