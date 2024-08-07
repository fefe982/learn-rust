// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/
// 3130. Find All Possible Stable Binary Arrays II
pub struct Solution;
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;
        let m = 1000000007i64;
        let mut dp = vec![vec![vec![0; one + 1]; zero + 1]; 2];
        for i in 1..=zero.min(limit) {
            dp[0][i][0] = 1;
        }
        for i in 1..=one.min(limit) {
            dp[1][0][i] = 1;
        }
        for i in 1..=zero {
            for j in 1..=one {
                dp[0][i][j] = dp[0][i - 1][j] + dp[1][i - 1][j];
                if i > limit {
                    dp[0][i][j] -= dp[1][i - limit - 1][j];
                }
                dp[0][i][j] = (dp[0][i][j] + m) % m;
                dp[1][i][j] = dp[0][i][j - 1] + dp[1][i][j - 1];
                if j > limit {
                    dp[1][i][j] -= dp[0][i][j - limit - 1];
                }
                dp[1][i][j] = (dp[1][i][j] + m) % m;
            }
        }
        ((dp[0][zero][one] + dp[1][zero][one]) % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_stable_arrays() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }
}
