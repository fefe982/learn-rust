// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/
// 3573. Best Time to Buy and Sell Stock V
pub struct Solution;
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = prices.len();
        let mut dp = vec![vec![0; 3]; k + 1];
        for i in 1..=k {
            dp[i][1] = prices[0] as i64;
            dp[i][2] = -(prices[0] as i64);
        }
        for i in 1..n {
            let p = prices[i] as i64;
            for j in (1..=k).rev() {
                dp[j][0] = dp[j][0].max(dp[j][1] - p).max(dp[j][2] + p);
                dp[j][1] = dp[j][1].max(dp[j - 1][0] + p);
                dp[j][2] = dp[j][2].max(dp[j - 1][0] - p);
            }
        }
        dp[k][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_profit() {
        assert_eq!(Solution::maximum_profit(vec![6, 11, 1, 5, 3, 15, 8], 3), 22);
        assert_eq!(Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2), 14);
        assert_eq!(Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36);
        assert_eq!(Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36);
    }
}
