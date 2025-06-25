// https://leetcode.com/problems/selling-pieces-of-wood/
// 2312. Selling Pieces of Wood
pub struct Solution;
impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for price in prices {
            dp[price[0] as usize][price[1] as usize] = price[2] as i64;
        }
        for i in 1..=m {
            for j in 1..=n {
                for k in 1..=i / 2 {
                    dp[i][j] = dp[i][j].max(dp[k][j] + dp[i - k][j]);
                }
                for k in 1..=j / 2 {
                    dp[i][j] = dp[i][j].max(dp[i][k] + dp[i][j - k]);
                }
            }
        }
        dp[m][n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_selling_wood() {
        assert_eq!(
            Solution::selling_wood(3, 5, vec_vec![[1, 4, 2], [2, 2, 7], [2, 1, 3]]),
            19
        );
        assert_eq!(
            Solution::selling_wood(4, 6, vec_vec![[3, 2, 10], [1, 4, 2], [4, 1, 3]]),
            32
        )
    }
}
