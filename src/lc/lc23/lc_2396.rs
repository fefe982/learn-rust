// https://leetcode.com/problems/strictly-palindromic-number/
// 2396. Strictly Palindromic Number
pub struct Solution;
impl Solution {
    pub fn is_strictly_palindromic(_n: i32) -> bool {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_strictly_palindromic() {
        assert_eq!(Solution::is_strictly_palindromic(9), false);
        assert_eq!(Solution::is_strictly_palindromic(4), false);
    }
}
