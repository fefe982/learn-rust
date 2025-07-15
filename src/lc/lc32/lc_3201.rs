// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/
// 3201. Find the Maximum Length of Valid Subsequence I
pub struct Solution;
use super::lc_3202;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        lc_3202::Solution::maximum_length(nums, 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_length() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4]), 4);
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
        assert_eq!(Solution::maximum_length(vec![1, 3]), 2);
    }
}
