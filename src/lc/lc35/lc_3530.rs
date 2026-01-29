// https://leetcode.com/problems/maximum-profit-from-valid-topological-order-in-dag/
// 3530. Maximum Profit From Valid Topological Order in DAG
pub struct Solution;
impl Solution {
    pub fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut g = vec![0; n];
        for e in edges {
            g[e[1] as usize] |= 1 << e[0];
        }
        let mut dp = vec![-1; 1 << n];
        dp[0] = 0;
        for mask in 1usize..(1 << n) {
            let mut save_mask = mask;
            while save_mask > 0 {
                let j = save_mask.trailing_zeros() as usize;
                let jmask = 1 << j;
                save_mask ^= jmask;
                let imask = mask ^ jmask;
                if g[j] & mask != g[j] || dp[imask] < 0 {
                    continue;
                }
                dp[mask] = dp[mask].max(dp[imask] + score[j] * (mask.count_ones() as i32));
            }
        }
        dp[(1 << n) - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(2, vec_vec![[0, 1]], vec![2, 3]), 8);
        assert_eq!(Solution::max_profit(3, vec_vec![[0, 1], [0, 2]], vec![1, 6, 3]), 25);
    }
}
