// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-i/
// 3095. Shortest Subarray With OR at Least K I
pub struct Solution;
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        super::lc_3097::Solution::minimum_subarray_length(nums, k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_subarray_length() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
