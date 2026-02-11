// https://leetcode.com/problems/longest-balanced-subarray-i/
// 3719. Longest Balanced Subarray I
pub struct Solution;
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        super::lc_3721::Solution::longest_balanced(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_balanced() {
        assert_eq!(Solution::longest_balanced(vec![9, 20, 5, 11, 20, 20]), 3);
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
