// https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected/
// 3363. Find the Maximum Number of Fruits Collected
pub struct Solution;
impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut dp = vec![vec![vec![0; n / 2 + 1]; n - 1]; 2];
        let mut sum = 0;
        for i in 0..n - 1 {
            for j in 0..=i.min(n - 2 - i) {
                if i > 0 {
                    if j > 0 {
                        dp[0][i][j] = dp[0][i - 1][j - 1];
                        dp[1][i][j] = dp[1][i - 1][j - 1];
                    }
                    dp[0][i][j] = dp[0][i][j].max(dp[0][i - 1][j]).max(dp[0][i - 1][j + 1]) + fruits[i][n - 1 - j];
                    dp[1][i][j] = dp[1][i][j].max(dp[1][i - 1][j]).max(dp[1][i - 1][j + 1]) + fruits[n - 1 - j][i]
                } else {
                    dp[0][i][j] = fruits[i][n - 1];
                    dp[1][i][j] = fruits[n - 1][i];
                }
            }
            sum += fruits[i][i];
        }
        sum + fruits[n - 1][n - 1] + dp[0][n - 2][0] + dp[1][n - 2][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_collected_fruits() {
        assert_eq!(
            Solution::max_collected_fruits(vec_vec![[1, 2, 3, 4], [5, 6, 8, 7], [9, 10, 11, 12], [13, 14, 15, 16]]),
            100
        );
        assert_eq!(Solution::max_collected_fruits(vec_vec![[1, 1], [1, 1]]), 4);
        assert_eq!(Solution::max_collected_fruits(vec_vec![[1, 1], [1, 1]]), 4);
    }
}
