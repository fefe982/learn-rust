// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
// 122. Best Time to Buy and Sell Stock II
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut last_buy = i32::MAX;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] && prices[i - 1] < last_buy {
                last_buy = prices[i - 1];
            }
            if prices[i] > last_buy && (i == prices.len() - 1 || prices[i] > prices[i + 1]) {
                max_profit += prices[i] - last_buy;
                last_buy = i32::MAX;
            }
        }
        max_profit
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
