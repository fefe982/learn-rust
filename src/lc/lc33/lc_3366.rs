// https://leetcode.com/problems/minimum-array-sum/
// 3366. Minimum Array Sum
pub struct Solution;
impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let op1 = op1 as usize;
        let op2 = op2 as usize;
        let mut dp = vec![vec![0; op2 + 1]; op1 + 1];
        for n in nums {
            let mut ndp = vec![vec![i32::MAX; op2 + 1]; op1 + 1];
            for i in 0..=op1 {
                for j in 0..=op2 {
                    ndp[i][j] = ndp[i][j].min(dp[i][j] + n);
                    if i > 0 {
                        ndp[i][j] = ndp[i][j].min(dp[i - 1][j] + (n + 1) / 2);
                    }
                    if j > 0 && n >= k {
                        ndp[i][j] = ndp[i][j].min(dp[i][j - 1] + n - k);
                    }
                    if i > 0 && j > 0 {
                        if n >= 2 * k - 1 {
                            ndp[i][j] = ndp[i][j].min(dp[i - 1][j - 1] + (n + 1) / 2 - k);
                        } else if n > k {
                            ndp[i][j] = ndp[i][j].min(dp[i - 1][j - 1] + (n - k + 1) / 2);
                        }
                    }
                }
            }
            dp = ndp
        }
        dp[op1][op2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_array_sum() {
        assert_eq!(Solution::min_array_sum(vec![2, 8, 3, 19, 3], 3, 1, 1), 23);
        assert_eq!(Solution::min_array_sum(vec![2, 4, 3], 3, 2, 1), 3);
    }
}
