// https://leetcode.com/problems/maximize-palindrome-length-from-subsequences/
// 1771. Maximize Palindrome Length From Subsequences
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let l1 = word1.len();
        let w = word1.chars().chain(word2.chars()).collect::<Vec<_>>();
        let mut left = vec![usize::MAX; 26];
        let mut right = vec![0; 26];
        let mut dp = vec![vec![0; w.len() + 1]; w.len() + 1];
        for i in 0..w.len() {
            let idx = w[i] as usize - 'a' as usize;
            if left[idx] == usize::MAX {
                left[idx] = i;
            }
            right[idx] = i;
            dp[i + 1][i] = -1;
        }
        let mut max = 0;
        for len in 1..=w.len() {
            for i in 0..w.len() - len + 1 {
                if w[i] == w[i + len - 1] {
                    dp[i][i + len] = dp[i + 1][i + len - 1] + 2;
                    if left[w[i] as usize - 'a' as usize] < l1 && right[w[i + len - 1] as usize - 'a' as usize] >= l1 {
                        max = max.max(dp[i][i + len]);
                    }
                } else {
                    dp[i][i + len] = dp[i][i + len - 1].max(dp[i + 1][i + len]);
                }
            }
        }
        if max == 1 {
            0
        } else {
            max
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_palindrom() {
        assert_eq!(Solution::longest_palindrome("ab".to_string(), "aa".to_string()), 3);
        assert_eq!(Solution::longest_palindrome("cacb".to_string(), "cbba".to_string()), 5);
        assert_eq!(Solution::longest_palindrome("ab".to_string(), "ab".to_string()), 3);
        assert_eq!(Solution::longest_palindrome("aa".to_string(), "bb".to_string()), 0);
    }
}
