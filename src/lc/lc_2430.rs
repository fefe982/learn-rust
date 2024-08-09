// https://leetcode.com/problems/maximum-deletions-on-a-string/
// 2430. Maximum Deletions on a String
pub struct Solution;
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes().iter().map(|x| (x - b'a') as i64).collect::<Vec<_>>();
        let mut dp = vec![1; s.len()];
        let m = 1_0000_0000_7;
        let n = s.len();
        for i in (0..n - 1).rev() {
            let l = n - i;
            let mut h1 = s[i];
            let mut h2 = s[i + 1];
            let mut mul = 1;
            let mut mono = true;
            for j in 0..l / 2 {
                if j > 0 {
                    h2 = (h2 - s[i + j] * mul) % m;
                    h2 = (h2 + m) % m;
                    h2 = ((h2 * 26 + s[i + 2 * j]) * 26 + s[i + 2 * j + 1]) % m;
                    h1 = (h1 * 26 + s[i + j]) % m;
                    if s[i + j] != s[i] {
                        mono = false;
                    }
                    mul = (mul * 26) % m;
                }
                if h1 == h2 && (mono || s[i..i + j + 1] == s[i + j + 1..i + 2 * j + 2]) {
                    dp[i] = dp[i].max(dp[i + j + 1] + 1);
                }
            }
        }
        dp[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delete_string() {
        assert_eq!(Solution::delete_string("abcabcd".to_string()), 2);
        assert_eq!(Solution::delete_string("aaabaab".to_string()), 4);
        assert_eq!(Solution::delete_string("aaaaa".to_string()), 5);
    }
}
