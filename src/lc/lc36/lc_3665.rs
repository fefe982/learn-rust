// https://leetcode.com/problems/twisted-mirror-path-count/
// 3665. Twisted Mirror Path Count
pub struct Solution;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![[0, 0]; n];
        dp[0][0] = 1;
        dp[0][1] = 1;
        const MOD: i64 = 1000000007;
        for j in 1..n {
            if grid[0][j - 1] == 0 {
                dp[j][0] = dp[j - 1][0];
            } else {
                break;
            }
        }
        for i in 1..grid.len() {
            let mut ndp = vec![[0, 0]; n];
            if grid[i - 1][0] == 0 {
                ndp[0][1] = dp[0][1];
            }
            for j in 1..n {
                if grid[i - 1][j] == 0 {
                    ndp[j][1] = (dp[j][1] + dp[j][0]) % MOD;
                } else {
                    ndp[j][1] = dp[j][0];
                }
                if grid[i][j - 1] == 0 {
                    ndp[j][0] = (ndp[j - 1][0] + ndp[j - 1][1]) % MOD;
                } else {
                    ndp[j][0] = ndp[j - 1][1];
                }
            }
            dp = ndp;
        }
        ((dp[n - 1][0] + dp[n - 1][1]) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn unique_paths() {
        assert_eq!(Solution::unique_paths(vec_vec![[0, 1, 0], [0, 0, 1], [1, 0, 0]]), 5);
        assert_eq!(Solution::unique_paths(vec_vec![[0, 0], [0, 0]]), 2);
        assert_eq!(Solution::unique_paths(vec_vec![[0, 1, 1], [1, 1, 0]]), 1);
    }
}
