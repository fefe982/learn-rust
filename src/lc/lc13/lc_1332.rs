// https://leetcode.com/problems/remove-palindromic-subsequences/
// 1332. Remove Palindromic Subsequences
pub struct Solution;
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                return 2;
            }
        }
        1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_palindrome_sub() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
    }
}
