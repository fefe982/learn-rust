// https://leetcode.com/problems/maximum-ascending-subarray-sum/
// 1800. Maximum Ascending Subarray Sum
pub struct Solution;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                res = res.max(sum);
                sum = nums[i];
            }
        }
        res.max(sum)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_ascending_sum() {
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
    }
}
