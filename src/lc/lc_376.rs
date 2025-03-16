// https://leetcode.com/problems/wiggle-subsequence/
// 376. Wiggle Subsequence
pub struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut p = 1;
        let mut v = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                p = v + 1;
            } else if nums[i] < nums[i - 1] {
                v = p + 1;
            }
        }
        p.max(v)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wiggle_max_length() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
        assert_eq!(Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
    }
}
