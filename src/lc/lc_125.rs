// https://leetcode.com/problems/valid-palindrome/
// 125. Valid Palindrome
pub struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            while i < j && !(s[i].is_ascii_alphanumeric()) {
                i += 1;
            }
            while i < j && !(s[j].is_ascii_alphanumeric()) {
                j -= 1;
            }
            if i == j {
                break;
            }
            if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
        assert_eq!(Solution::is_palindrome("a.".to_string()), true);
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
