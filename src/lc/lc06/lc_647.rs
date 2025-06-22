// https://leetcode.com/problems/palindromic-substrings/
// 647. Palindromic Substrings
pub struct Solution;
impl Solution {
    fn palindrome(s: &[u8]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = 0;
        for i in 1..=s.len() {
            for j in 0..i {
                if Self::palindrome(&s[j..i]) {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_substrings() {
        assert_eq!(Solution::count_substrings(String::from("abc")), 3);
        assert_eq!(Solution::count_substrings(String::from("aaa")), 6);
    }
}
