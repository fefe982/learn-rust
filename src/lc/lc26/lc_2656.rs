// https://leetcode.com/problems/maximum-sum-with-exactly-k-elements/
// 2656. Maximum Sum With Exactly K Elements
pub struct Solution;
impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        (2 * *nums.iter().max().unwrap() + k - 1) * k / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximize_sum() {
        assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(Solution::maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
