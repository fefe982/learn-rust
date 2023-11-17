// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/
// 1877. Minimize Maximum Pair Sum in Array
pub struct Solution;
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut max = 0;
        for i in 0..n / 2 {
            max = max.max(nums[i] + nums[n - 1 - i]);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_pair_sum() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
