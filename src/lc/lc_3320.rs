// https://leetcode.com/problems/count-the-number-of-winning-sequences/
// 3320. Count the Number of Winning Sequences
pub struct Solution;
impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        let len = s.len();
        let offset = len + 1;
        let mut dp = vec![vec![0; 3]; offset * 2 + 1];
        let m = 1_000_000_007i64;
        for (i, c) in s.chars().enumerate() {
            let ic = match c {
                'F' => 0,
                'W' => 1,
                'E' => 2,
                _ => 3,
            };
            let wic = (ic + 1) % 3;
            let lic = (ic + 2) % 3;
            let mut ndp = vec![vec![0; 3]; offset * 2 + 1];
            if i == 0 {
                dp[offset + 1][wic] = 1;
                dp[offset][ic] = 1;
                dp[offset - 1][lic] = 1;
            } else {
                for j in offset - i - 1..=offset + i + 1 {
                    ndp[j][wic] = (ndp[j][wic] + dp[j - 1][ic] + dp[j - 1][lic]) % m;
                    ndp[j][ic] = (ndp[j][ic] + dp[j][wic] + dp[j][lic]) % m;
                    ndp[j][lic] = (ndp[j][lic] + dp[j + 1][ic] + dp[j + 1][wic]) % m;
                }
                dp = ndp;
            }
        }
        let mut ans = 0;
        for i in 1..=len {
            ans = (ans + dp[offset + i][0] + dp[offset + i][1] + dp[offset + i][2]) % m;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_winning_sequences() {
        assert_eq!(Solution::count_winning_sequences("FFF".to_string()), 3);
        assert_eq!(Solution::count_winning_sequences("FWEFW".to_string()), 18);
    }
}
