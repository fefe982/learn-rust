// https://leetcode.com/problems/palindrome-partitioning-ii/
// 132. Palindrome Partitioning II
pub struct Solution;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = -1;
        for i in 0..n {
            for j in 0..=i.min(n - 1 - i) {
                if s[i - j] == s[i + j] {
                    dp[i + j + 1] = dp[i + j + 1].min(dp[i - j] + 1);
                } else {
                    break;
                }
            }
            for j in 0..(i + 1).min(n - 1 - i) {
                if s[i - j] == s[i + j + 1] {
                    dp[i + j + 2] = dp[i + j + 2].min(dp[i - j] + 1);
                } else {
                    break;
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cut() {
        assert_eq!(Solution::min_cut(String::from("aab")), 1);
        assert_eq!(Solution::min_cut(String::from("a")), 0);
        assert_eq!(Solution::min_cut(String::from("ab")), 1);
    }
}
