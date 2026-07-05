// https://leetcode.com/problems/count-distinct-ways-to-form-target-from-two-strings/
// 3981. Count Distinct Ways to Form a Target String in a Binary Matrix
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn substring_count(word: &[u8], target: &[u8]) -> i64 {
        let mut dp = vec![0; target.len() + 1];
        dp[0] = 1;
        for i in 0..word.len() {
            for j in (1..=target.len()).rev() {
                if word[i] == target[j - 1] {
                    dp[j] = (dp[j] + dp[j - 1]) % MOD;
                }
            }
        }
        dp[target.len()]
    }
    pub fn interleave_characters(word1: String, word2: String, target: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let target = target.as_bytes();
        let len1 = word1.len();
        let len2 = word2.len();
        let len3 = target.len();
        let mut dp = vec![vec![vec![0; len2 + 2]; len1 + 2]; len3 + 1];
        for i in 1..=len1 + 1 {
            for j in 1..=len2 + 1 {
                dp[0][i][j] = 1;
            }
        }
        for i in 0..len3 {
            for j in 0..=len1 {
                for k in 0..=len2 {
                    let mut r = dp[i + 1][j][k + 1] + dp[i + 1][j + 1][k] - dp[i + 1][j][k] + MOD;
                    if j > 0 && word1[j - 1] == target[i] {
                        r += dp[i][j][k + 1] - dp[i][j][k] + MOD;
                    }
                    if k > 0 && word2[k - 1] == target[i] {
                        r += dp[i][j + 1][k] - dp[i][j][k] + MOD;
                    }
                    dp[i + 1][j + 1][k + 1] = r % MOD;
                }
            }
        }
        ((dp[len3][len1 + 1][len2 + 1] - Self::substring_count(word1, target) - Self::substring_count(word2, target)
            + 2 * MOD)
            % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interleave_characters() {
        assert_eq!(
            Solution::interleave_characters("abc".to_string(), "bac".to_string(), "abc".to_string()),
            5
        );
        assert_eq!(
            Solution::interleave_characters("cd".to_string(), "cd".to_string(), "ccd".to_string()),
            4
        );
        assert_eq!(
            Solution::interleave_characters("xy".to_string(), "xy".to_string(), "xyxy".to_string()),
            2
        );
        assert_eq!(
            Solution::interleave_characters("ab".to_string(), "cde".to_string(), "ace".to_string()),
            1
        );
    }
}
