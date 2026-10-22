// https://leetcode.com/problems/minimum-cost-path-with-alternating-directions-ii/
// 3603. Minimum Cost Path with Alternating Directions II
pub struct Solution;
impl Solution {
    pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![0; n];
        let mut wait_cost = wait_cost;
        wait_cost[0][0] = 0;
        dp[0] = 1;
        for j in 1..n {
            dp[j] = dp[j - 1] + wait_cost[0][j - 1] as i64 + j as i64 + 1;
        }
        for i in 1..m {
            let mut ndp = vec![0; n];
            ndp[0] = dp[0] + wait_cost[i - 1][0] as i64 + i as i64 + 1;
            for j in 1..n {
                ndp[j] = (ndp[j - 1] + wait_cost[i][j - 1] as i64).min(dp[j] + wait_cost[i - 1][j] as i64)
                    + (i as i64 + 1) * (j as i64 + 1);
            }
            dp = ndp;
        }
        dp[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(1, 2, vec_vec![[1, 2]]), 3);
        assert_eq!(Solution::min_cost(2, 2, vec_vec![[3, 5], [2, 4]]), 9);
        assert_eq!(Solution::min_cost(2, 3, vec_vec![[6, 1, 4], [3, 2, 5]]), 16);
    }
}
