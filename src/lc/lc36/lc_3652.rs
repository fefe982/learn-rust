// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/
// 3652. Best Time to Buy and Sell Stock Using Strategy
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0;
        let k = k as usize;
        for i in 0..k / 2 {
            let p = prices[i] as i64;
            sum += p * strategy[i] as i64;
        }
        let mut diff = -sum;
        for i in k / 2..k {
            let p = prices[i] as i64;
            sum += p * strategy[i] as i64;
            diff += p * (1 - strategy[i]) as i64;
        }
        let mut max_diff = diff;
        for i in k..prices.len() {
            let p = prices[i] as i64;
            sum += p * strategy[i] as i64;
            diff +=
                p * (1 - strategy[i]) as i64 + prices[i - k] as i64 * strategy[i - k] as i64 - prices[i - k / 2] as i64;
            max_diff = max_diff.max(diff);
        }
        sum + max_diff.max(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10);
        assert_eq!(Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9);
    }
}
