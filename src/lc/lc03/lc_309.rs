// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// 309. Best Time to Buy and Sell Stock with Cooldown
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }
        let mut dp = vec![vec![0; prices.len()]; 3];
        dp[0][0] = 0;
        dp[1][0] = 0;
        dp[2][0] = -prices[0];
        dp[0][1] = prices[1] - prices[0];
        dp[1][1] = 0;
        dp[2][1] = dp[2][0].max(-prices[1]);
        for i in 2..prices.len() {
            dp[1][i] = dp[0][i - 1].max(dp[1][i - 1]);
            dp[0][i] = dp[2][i - 1] + prices[i];
            dp[2][i] = (dp[1][i - 1] - prices[i]).max(dp[2][i - 1]);
        }
        dp[0][prices.len() - 1].max(dp[1][prices.len() - 1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
