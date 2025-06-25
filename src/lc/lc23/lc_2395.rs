// https://leetcode.com/problems/find-subarrays-with-equal-sum/
// 2395. Find Subarrays With Equal Sum
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return false;
        }
        let mut appeared = HashSet::new();
        for idx in 0..(nums.len() - 1) {
            let sum = nums[idx] + nums[idx + 1];
            if !appeared.insert(sum) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_subarrays() {
        assert_eq!(Solution::find_subarrays(vec![4, 2, 4]), true);
        assert_eq!(Solution::find_subarrays(vec![1, 2, 3, 4, 5]), false);
        assert_eq!(Solution::find_subarrays(vec![0, 0, 0]), true);
    }
}
