// https://leetcode.com/problems/maximum-difference-between-increasing-elements/
// 2016. Maximum Difference Between Increasing Elements
pub struct Solution;
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut min = nums[0];
        for i in 1..nums.len() {
            if nums[i] > min {
                max = max.max(nums[i] - min);
            } else {
                min = nums[i];
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_difference() {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4, 2]), 4);
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
