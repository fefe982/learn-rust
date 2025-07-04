// https://leetcode.cn/problems/break-a-palindrome/
// 1328. Break a Palindrome
pub struct Solution;
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut palindrome = palindrome.chars().collect::<Vec<_>>();
        let n = palindrome.len();
        if n == 1 {
            return "".to_string();
        }
        for i in 0..n / 2 {
            if palindrome[i] != 'a' {
                palindrome[i] = 'a';
                return palindrome.iter().collect();
            }
        }
        palindrome[n - 1] = 'b';
        palindrome.iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn break_palindrome() {
        assert_eq!(Solution::break_palindrome("aba".to_string()), "abb".to_string());
        assert_eq!(Solution::break_palindrome("aa".to_string()), "ab".to_string());
        assert_eq!(Solution::break_palindrome("abccba".to_string()), "aaccba".to_string());
        assert_eq!(Solution::break_palindrome("a".to_string()), "".to_string());
    }
}
