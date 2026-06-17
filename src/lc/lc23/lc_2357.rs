// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/
// 2357. Make Array Zero by Subtracting Equal Amounts
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.dedup();
        if nums[0] == 0 {
            (nums.len() - 1) as i32
        } else {
            nums.len() as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }
}
