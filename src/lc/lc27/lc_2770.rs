// https://leetcode.com/problems/maximum-number-of-jumps-to-reach-the-last-index/
// 2770. Maximum Number of Jumps to Reach the Last Index
pub struct Solution;
impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![-1; n];
        dp[0] = 0;
        for i in 1..n {
            for j in (0..i).rev() {
                if dp[j] != -1 && (nums[i] - nums[j]).abs() <= target {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_jumps() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
    }
}
