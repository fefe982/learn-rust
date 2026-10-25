// https://leetcode.com/problems/maximum-median-sum-of-subsequences-of-size-3/
// 3627. Maximum Median Sum of Subsequences of Size 3
pub struct Solution;
impl Solution {
    pub fn maximum_median_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let n = nums.len() / 3;
        for i in (n..nums.len()).step_by(2) {
            sum += nums[i] as i64;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_median_sum() {
        assert_eq!(Solution::maximum_median_sum(vec![2, 1, 3, 2, 1, 3]), 5);
        assert_eq!(Solution::maximum_median_sum(vec![1, 1, 10, 10, 10, 10]), 20);
    }
}
