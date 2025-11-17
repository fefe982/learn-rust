// https://leetcode.cn/problems/zheng-ze-biao-da-shi-pi-pei-lcof/
// LCR 137. 模糊搜索验证
pub struct Solution;
impl Solution {
    pub fn article_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let ls = s.len();
        let lp = p.len();
        let mut dp = vec![vec![false; lp + 1]; ls + 1];
        dp[0][0] = true;
        for j in 0..lp {
            if p[j] == b'*' {
                dp[0][j + 1] = dp[0][j - 1];
            }
        }
        for i in 0..ls {
            for j in 0..lp {
                if s[i] == p[j] || p[j] == b'.' {
                    dp[i + 1][j + 1] = dp[i][j];
                } else if p[j] == b'*' {
                    if s[i] == p[j - 1] || p[j - 1] == b'.' {
                        dp[i + 1][j + 1] = dp[i + 1][j - 1] || dp[i][j + 1];
                    } else {
                        dp[i + 1][j + 1] = dp[i + 1][j - 1];
                    }
                }
            }
        }
        dp[ls][lp]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn article_match() {
        assert_eq!(Solution::article_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::article_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::article_match("ab".to_string(), ".*".to_string()), true);
    }
}
