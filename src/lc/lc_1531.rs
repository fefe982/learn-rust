// https://leetcode.com/problems/string-compression-ii/
// 1531. String Compression II
pub struct Solution;
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; s.len() + 1];
        let s = s.as_bytes();
        for i in 1..=s.len() {
            let c = s[i - 1];
            for j in 0..=(k as usize).min(i) {
                if j > 0 {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 500;
                }
                let mut l = 0;
                let mut d = 0usize;
                for r in (0..i).rev() {
                    if s[r] != c {
                        d += 1;
                    } else {
                        l += 1;
                    }
                    if d > j as usize {
                        break;
                    }
                    let mut ndp = dp[i - l - d][j - d] + 1;
                    if l > 1 {
                        ndp += 1;
                    }
                    if l >= 10 {
                        ndp += 1;
                    }
                    if l >= 100 {
                        ndp += 1;
                    }
                    if l >= 1000 {
                        ndp += 1;
                    }
                    dp[i][j] = dp[i][j].min(ndp);
                }
            }
        }
        dp[s.len()][k as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_length_of_optimal_compression() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2),
            4
        );
        assert_eq!(Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2), 2);
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0),
            3
        );
    }
}
