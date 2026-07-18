// https://leetcode.com/problems/partition-string-into-minimum-beautiful-substrings/
// 2767. Partition String Into Minimum Beautiful Substrings
pub struct Solution;
impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![n as i32 + 1; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            if s[i - 1] == b'0' {
                continue;
            }
            let mut val = 0;
            for j in i..=n {
                val = val * 2 + (s[j - 1] - b'0') as i32;
                if 15625 % val == 0 {
                    dp[j] = dp[j].min(dp[i - 1] + 1);
                }
            }
        }
        if dp[n] > n as i32 {
            -1
        } else {
            dp[n]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_beautiful_substrings() {
        assert_eq!(Solution::minimum_beautiful_substrings("1011".to_string()), 2);
        assert_eq!(Solution::minimum_beautiful_substrings("111".to_string()), 3);
        assert_eq!(Solution::minimum_beautiful_substrings("0".to_string()), -1);
    }
}
