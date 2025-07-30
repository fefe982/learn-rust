// https://leetcode.com/problems/lexicographically-smallest-palindrome/
// 2697. Lexicographically Smallest Palindrome
pub struct Solution;
impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut r = vec![b' '; s.len()];
        for i in 0..s.len() / 2 {
            if s[i] < s[s.len() - 1 - i] {
                r[i] = s[i];
            } else {
                r[i] = s[s.len() - 1 - i];
            }
            r[s.len() - 1 - i] = r[i];
        }
        if s.len() % 2 == 1 {
            r[s.len() / 2] = s[s.len() / 2];
        }
        String::from_utf8(r).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_smallest_palindrome() {
        assert_eq!(Solution::make_smallest_palindrome("egcfe".to_string()), "efcfe");
        assert_eq!(Solution::make_smallest_palindrome("abcd".to_string()), "abba");
        assert_eq!(Solution::make_smallest_palindrome("seven".to_string()), "neven");
    }
}
