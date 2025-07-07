// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
// 1752. Check if Array Is Sorted and Rotated
pub struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut i = 1;
        while i < nums.len() && nums[i - 1] <= nums[i] {
            i += 1;
        }
        if i == nums.len() {
            return true;
        }
        i += 1;
        while i < nums.len() && nums[i - 1] <= nums[i] {
            i += 1;
        }
        if i != nums.len() {
            return false;
        }
        nums[i - 1] <= nums[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(Solution::check(vec![3, 4, 5, 1, 2]), true);
        assert_eq!(Solution::check(vec![2, 1, 3, 4]), false);
        assert_eq!(Solution::check(vec![1, 2, 3]), true);
    }
}
