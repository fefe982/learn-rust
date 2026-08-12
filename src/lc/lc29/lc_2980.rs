// https://leetcode.com/problems/check-if-bitwise-or-has-trailing-zeros/
// 2980. Check if Bitwise OR Has Trailing Zeros
pub struct Solution;
impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        nums.into_iter().filter(|&x| x & 1 == 0).count() > 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_trailing_zeros() {
        assert_eq!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]), true);
        assert_eq!(Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]), false);
    }
}
