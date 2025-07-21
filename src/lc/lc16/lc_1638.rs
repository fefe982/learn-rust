// https://leetcode.com/problems/count-substrings-that-differ-by-one-character/
// 1638. Count Substrings That Differ by One Character
pub struct Solution;
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![vec![0, 0]; t.len() + 1]; s.len() + 1];
        let mut sum = 0;
        for i in 0..s.len() {
            for j in 0..t.len() {
                if s[i] == t[j] {
                    dp[i + 1][j + 1][0] = dp[i][j][0] + 1;
                    dp[i + 1][j + 1][1] = dp[i][j][1];
                } else {
                    dp[i + 1][j + 1][1] = dp[i][j][0] + 1;
                }
                sum += dp[i + 1][j + 1][1];
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_substrings() {
        assert_eq!(
            Solution::count_substrings(String::from("aba"), String::from("baba")),
            6
        );
        assert_eq!(
            Solution::count_substrings(String::from("ab"), String::from("bb")),
            3
        );
    }
}
