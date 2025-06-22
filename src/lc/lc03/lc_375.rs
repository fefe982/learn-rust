// https://leetcode.com/problems/guess-number-higher-or-lower-ii/
// 375. Guess Number Higher or Lower II
pub struct Solution;
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 2..=n {
            for j in 1..=n - i + 1 {
                dp[j][j + i - 1] = (j as i32 + dp[j + 1][j + i - 1]).min(dp[j][j + i - 2] + (j + i - 1) as i32);
                for k in j + 1..j + i - 1 {
                    dp[j][j + i - 1] = dp[j][j + i - 1].min(k as i32 + dp[k + 1][j + i - 1].max(dp[j][k - 1]))
                }
            }
        }
        dp[1][n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_money_amount() {
        assert_eq!(Solution::get_money_amount(10), 16);
        assert_eq!(Solution::get_money_amount(1), 0);
        assert_eq!(Solution::get_money_amount(2), 1);
    }
}
