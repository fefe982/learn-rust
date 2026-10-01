// https://leetcode.com/problems/longest-palindromic-subsequence-after-at-most-k-operations/
// 3472. Longest Palindromic Subsequence After at Most K Operations
pub struct Solution;
impl Solution {
    fn dp(s: &[u8], i: usize, j: usize, k: i32, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i > j {
            return 0;
        }
        if i == j {
            return 1;
        }
        if memo[i][j][k as usize] != -1 {
            return memo[i][j][k as usize];
        }
        let mut res;
        if s[i] == s[j] {
            res = 2 + Self::dp(s, i + 1, j - 1, k, memo);
        } else {
            res = Self::dp(s, i + 1, j, k, memo).max(Self::dp(s, i, j - 1, k, memo));
            let diff = (s[i] as i32 - s[j] as i32).abs();
            let min = diff.min(26 - diff);
            if k >= min {
                res = res.max(2 + Self::dp(s, i + 1, j - 1, k - min, memo));
            }
        }
        memo[i][j][k as usize] = res;
        res
    }
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut memo = vec![vec![vec![-1; (k + 1) as usize]; n]; n];
        Self::dp(s, 0, n - 1, k, &mut memo)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_palindromic_subsequence() {
        assert_eq!(Solution::longest_palindromic_subsequence("abced".to_string(), 2), 3);
        assert_eq!(Solution::longest_palindromic_subsequence("aaazzz".to_string(), 4), 6);
    }
}
