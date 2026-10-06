// https://leetcode.com/problems/longest-palindrome-after-substring-concatenation-i/
// 3503. Longest Palindrome After Substring Concatenation I
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        super::lc_3504::Solution::longest_palindrome(s, t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("a".to_string(), "a".to_string()), 2);
        assert_eq!(Solution::longest_palindrome("abc".to_string(), "def".to_string()), 1);
        assert_eq!(Solution::longest_palindrome("b".to_string(), "aaaa".to_string()), 4);
        assert_eq!(
            Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string()),
            5
        );
    }
}
