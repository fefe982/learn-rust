// https://leetcode.com/problems/running-sum-of-1d-array/
// 1480. Running Sum of 1d Array
pub struct Solution;
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn running_sum() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
