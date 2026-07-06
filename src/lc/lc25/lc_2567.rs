// https://leetcode.com/problems/minimum-score-by-changing-two-elements/
// 2567. Minimum Score by Changing Two Elements
pub struct Solution;
impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = i32::MAX;
        for i in 0..=2 {
            ans = ans.min(nums[n - 3 + i] - nums[i]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_sum() {
        assert_eq!(Solution::minimize_sum(vec![1, 4, 7, 8, 5]), 3);
        assert_eq!(Solution::minimize_sum(vec![1, 4, 3]), 0);
    }
}
