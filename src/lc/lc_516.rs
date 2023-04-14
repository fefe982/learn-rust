// https://leetcode.com/problems/longest-palindromic-subsequence/\
// 516. Longest Palindromic Subsequence
pub struct Solution;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let mut max_cnt = vec![vec![0; s.len()]; s.len()];
        for idx in 0..s.len() - 1 {
            max_cnt[idx][0] = 1;
            if s[idx] == s[idx + 1] {
                max_cnt[idx][1] = 2;
            } else {
                max_cnt[idx][1] = 1;
            };
        }
        max_cnt[s.len() - 1][0] = 1;
        for len in 2..s.len() {
            for idx in 0..s.len() - len {
                let max_eq = if s[idx] == s[idx + len] {
                    max_cnt[idx + 1][len - 2] + 2
                } else {
                    0
                };
                max_cnt[idx][len] = std::cmp::max(
                    max_eq,
                    std::cmp::max(max_cnt[idx][len - 1], max_cnt[idx + 1][len - 1]),
                );
            }
        }
        max_cnt[0][s.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_palindrome_subseq() {
        assert_eq!(
            Solution::longest_palindrome_subseq(String::from("bbbab")),
            4
        );
        assert_eq!(Solution::longest_palindrome_subseq(String::from("cbbd")), 2);
    }
}
