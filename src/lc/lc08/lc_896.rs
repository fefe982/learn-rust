// https://leetcode.com/problems/monotonic-array/
// 896. Monotonic Array
pub struct Solution;
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut inc = true;
        let mut dec = true;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                inc = false;
            }
            if nums[i] < nums[i + 1] {
                dec = false;
            }
        }
        inc || dec
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_monotonic() {
        assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
        assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
        assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false);
    }
}
