// https://leetcode.com/problems/minimum-number-of-operations-to-satisfy-conditions/
// 3122. Minimum Number of Operations to Satisfy a Given Integer Condition
pub struct Solution;
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 2]; 2];
        let mut maxd = 0;
        for i in (0..grid[0].len()).rev() {
            let ithis = i % 2;
            let ilast = 1 - ithis;
            let mut cnt = vec![dp[ilast][0]; 10];
            cnt[maxd] = dp[ilast][1];
            for j in 0..grid.len() {
                cnt[grid[j][i] as usize] += 1;
            }
            dp[ithis][0] = 0;
            dp[ithis][1] = 0;
            for j in 0..10 {
                if cnt[j] > dp[ithis][0] {
                    dp[ithis][1] = dp[ithis][0];
                    dp[ithis][0] = cnt[j];
                    maxd = j;
                } else if cnt[j] > dp[ithis][1] {
                    dp[ithis][1] = cnt[j];
                }
            }
        }
        ((grid.len() * grid[0].len()) - dp[0][0].max(dp[0][1])) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec_vec![[1, 0, 2], [1, 0, 2]]), 0);
        assert_eq!(Solution::minimum_operations(vec_vec![[1, 1, 1], [0, 0, 0]]), 3);
        assert_eq!(Solution::minimum_operations(vec_vec![[1], [2], [3]]), 2);
    }
}
