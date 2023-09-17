// https://leetcode.com/problems/house-robber-ii/
// 213. House Robber II
pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut max = 0;
        for i in 0..2 {
            let n = &nums[i..];
            let mut dp = vec![0; nums.len()];
            dp[1] = n[0];
            for i in 2..nums.len() {
                dp[i] = dp[i - 1].max(dp[i - 2] + n[i - 1]);
            }
            max = max.max(dp[nums.len() - 1]);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
}
