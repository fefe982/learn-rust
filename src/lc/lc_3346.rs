// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/
// 3346. Maximum Frequency of an Element After Performing Operations I
pub struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        super::lc_3347::Solution::max_frequency(nums, k, num_operations)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_frequency() {
        assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
        assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    }
}
