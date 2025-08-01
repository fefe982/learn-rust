// https://leetcode.com/problems/number-of-ways-to-earn-points/
// 2585. Number of Ways to Earn Points
pub struct Solution;
impl Solution {
    fn dp(target: i32, types: &Vec<Vec<i32>>, i: usize, j: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if target == 0 {
            return 1;
        }
        if target < 0 {
            return 0;
        }
        if dp[target as usize][i][j] != -1 {
            return dp[target as usize][i][j];
        }
        let mut ans = 0;
        if j < types[i][0] as usize {
            ans += Self::dp(target - types[i][1], types, i, j + 1, dp);
        }
        if i + 1 < types.len() {
            ans += Self::dp(target, types, i + 1, 0, dp);
        }
        ans %= 1_000_000_007;
        dp[target as usize][i][j] = ans;
        ans
    }
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        Self::dp(
            target,
            &types,
            0,
            0,
            &mut vec![vec![vec![-1; 51]; 51]; target as usize + 1],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_ways_to_reach_target() {
        assert_eq!(Solution::ways_to_reach_target(6, vec_vec![[6, 1], [3, 2], [2, 3]]), 7);
        assert_eq!(
            Solution::ways_to_reach_target(5, vec_vec![[50, 1], [50, 2], [50, 5]]),
            4
        );
    }
}
