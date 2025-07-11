// https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/
// 2472. Maximum Number of Non-overlapping Palindrome Substrings
pub struct Solution;
impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][i] = true;
            if i > 0 && i < n - 1 {
                let mut l = i - 1;
                let mut r = i + 1;
                while s[l] == s[r] {
                    dp[l][r] = true;
                    if l == 0 || r == n - 1 {
                        break;
                    }
                    l -= 1;
                    r += 1;
                }
            }
            if i < n - 1 && s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                if i > 0 && i + 2 < n {
                    let mut l = i - 1;
                    let mut r = i + 2;
                    while s[l] == s[r] {
                        dp[l][r] = true;
                        if l == 0 || r == n - 1 {
                            break;
                        }
                        l -= 1;
                        r += 1;
                    }
                }
            }
        }
        let mut cnt = vec![0; n + 1];
        for i in (0..=n - k as usize).rev() {
            cnt[i] = cnt[i + 1];
            for j in i + k as usize - 1..n {
                if dp[i][j] {
                    cnt[i] = cnt[i].max(cnt[j + 1] + 1);
                }
            }
        }
        cnt[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_palindromes() {
        assert_eq!(Solution::max_palindromes("abaccdbbd".to_string(), 3), 2);
        assert_eq!(Solution::max_palindromes("adbcda".to_string(), 2), 0);
    }
}
