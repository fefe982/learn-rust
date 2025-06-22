// https://leetcode.com/problems/largest-plus-sign/
// 764. Largest Plus Sign
pub struct Solution;
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![(0, 0); n]; n];
        for mine in mines {
            dp[mine[0] as usize][mine[1] as usize] = (-1, -1);
        }
        let mut ans = -1;
        for i in 0..n {
            for j in 0..n {
                if dp[i][j].0 == -1 {
                    continue;
                }
                if i > 0 {
                    dp[i][j].0 = dp[i - 1][j].0 + 1;
                }
                if j > 0 {
                    dp[i][j].1 = dp[i][j - 1].1 + 1;
                }
            }
        }
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if dp[i][j].0 == -1 {
                    continue;
                }
                if i < n - 1 {
                    dp[i][j].0 = dp[i][j].0.min(dp[i + 1][j].0 + 1);
                } else {
                    dp[i][j].0 = 0;
                }
                if j < n - 1 {
                    dp[i][j].1 = dp[i][j].1.min(dp[i][j + 1].1 + 1);
                } else {
                    dp[i][j].1 = 0;
                }
                ans = ans.max(dp[i][j].0.min(dp[i][j].1));
            }
        }
        ans + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn order_of_largest_plus_sign() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec_vec![[3, 0], [3, 3]]), 3);
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec_vec![[4, 2]]), 2);
        assert_eq!(Solution::order_of_largest_plus_sign(1, vec_vec![[0, 0]]), 0);
    }
}
