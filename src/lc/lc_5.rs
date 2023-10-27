// https://leetcode.com/problems/longest-palindromic-substring/
// 5. Longest Palindromic Substring
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut max_len = 0;
        let mut start = 0;
        for i in 0..s.len() {
            for j in 0..=i {
                dp[j][i] = if i - j < 2 {
                    s[j] == s[i]
                } else {
                    dp[j + 1][i - 1] && s[j] == s[i]
                };
                if dp[j][i] && max_len < i - j + 1 {
                    max_len = i - j + 1;
                    start = j;
                }
            }
        }
        s[start..start + max_len].iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
