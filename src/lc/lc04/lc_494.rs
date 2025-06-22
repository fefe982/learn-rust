// https://leetcode.com/problems/target-sum/
// 494. Target Sum
pub struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if (target + sum) % 2 == 1 || nums.is_empty() {
            return 0;
        }
        let target = (sum - target) / 2;
        if target < 0 {
            return 0;
        }
        let mut dp = vec![vec![0; target as usize + 1]; nums.len() + 1];
        dp[0][0] = 1;
        for i in 1..=nums.len() {
            for j in 0..=target as usize {
                let v = nums[i - 1] as usize;
                if j >= v {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - v];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        dp[nums.len()][target as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 0], 1), 2);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
