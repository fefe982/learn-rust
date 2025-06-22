// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
// 714. Best Time to Buy and Sell Stock with Transaction Fee
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = vec![vec![0; prices.len() + 1]; 2];
        dp[1][0] = i32::MIN / 2;
        for i in 0..prices.len() {
            dp[0][i + 1] = dp[0][i].max(dp[1][i] + prices[i] - fee);
            dp[1][i + 1] = (dp[0][i] - prices[i]).max(dp[1][i]);
        }
        dp[0][prices.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }
}
