// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
// 712. Minimum ASCII Delete Sum for Two Strings
pub struct Solution;
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i2 in 0..s2.len() {
            dp[0][i2 + 1] = dp[0][i2] + s2[i2] as i32;
        }
        for i1 in 0..s1.len() {
            dp[i1 + 1][0] = dp[i1][0] + s1[i1] as i32;
            for i2 in 0..s2.len() {
                if s1[i1] == s2[i2] {
                    dp[i1 + 1][i2 + 1] = dp[i1][i2];
                } else {
                    dp[i1 + 1][i2 + 1] = (dp[i1][i2 + 1] + s1[i1] as i32).min(dp[i1 + 1][i2] + s2[i2] as i32);
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_delete_sum() {
        assert_eq!(
            Solution::minimum_delete_sum(String::from("sea"), String::from("eat")),
            231
        );
        assert_eq!(
            Solution::minimum_delete_sum(String::from("delete"), String::from("leet")),
            403
        );
    }
}
