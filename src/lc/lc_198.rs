// https://leetcode.com/problems/house-robber/
// 198. House Robber
pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        for i in 2..nums.len() + 1 {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i - 1]);
        }
        dp[nums.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
