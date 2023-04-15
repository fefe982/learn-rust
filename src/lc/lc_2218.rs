// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/description/
// 2218. Maximum Value of K Coins From Piles
pub struct Solution;
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; piles.len()];
        dp[0][0] = piles[0][0];
        for idx in 1..k {
            if idx >= piles[0].len() {
                break;
            }
            dp[0][idx] = dp[0][idx - 1] + piles[0][idx];
        }
        let mut sum = vec![0; k];
        for p in 1..piles.len() {
            dp[p][0] = std::cmp::max(dp[p - 1][0], piles[p][0]);
            sum[0] = piles[p][0];
            for idx in 1..k {
                dp[p][idx] = dp[p - 1][idx];
                if idx < piles[p].len() {
                    sum[idx] = sum[idx - 1] + piles[p][idx];
                    dp[p][idx] = std::cmp::max(dp[p][idx], sum[idx]);
                }
                for j in 0..idx {
                    if j >= piles[p].len() {
                        break;
                    }
                    dp[p][idx] = std::cmp::max(dp[p][idx], dp[p - 1][idx - j - 1] + sum[j])
                }
            }
        }
        dp[piles.len() - 1][k - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_value_of_coins() {
        assert_eq!(
            Solution::max_value_of_coins(
                vec_vec![[37, 88], [51, 64, 65, 20, 95, 30, 26], [9, 62, 20], [44]],
                9
            ),
            494
        );
        assert_eq!(
            Solution::max_value_of_coins(vec_vec![[1, 100, 3], [7, 8, 9]], 2),
            101
        );
        assert_eq!(
            Solution::max_value_of_coins(
                vec_vec![
                    [100],
                    [100],
                    [100],
                    [100],
                    [100],
                    [100],
                    [1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }
}
