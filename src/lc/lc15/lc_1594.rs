// https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/
// 1594. Maximum Non Negative Product in a Matrix
pub struct Solution;
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![(-1, -1); n]; m];
        match grid[0][0].cmp(&0) {
            std::cmp::Ordering::Equal => {
                dp[0][0] = (0, 0);
            }
            std::cmp::Ordering::Greater => {
                dp[0][0] = (-1, grid[0][0] as i64);
            }
            std::cmp::Ordering::Less => {
                dp[0][0] = (-grid[0][0] as i64, -1);
            }
        }
        for i in 0..m {
            for j in 0..n {
                match grid[i][j].cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        dp[i][j] = (0, 0);
                    }
                    std::cmp::Ordering::Greater => {
                        if i > 0 {
                            dp[i][j] = (
                                dp[i][j].0.max(dp[i - 1][j].0 * grid[i][j] as i64),
                                dp[i][j].1.max(dp[i - 1][j].1 * grid[i][j] as i64),
                            );
                        }
                        if j > 0 {
                            dp[i][j] = (
                                dp[i][j].0.max(dp[i][j - 1].0 * grid[i][j] as i64),
                                dp[i][j].1.max(dp[i][j - 1].1 * grid[i][j] as i64),
                            );
                        }
                    }
                    std::cmp::Ordering::Less => {
                        if i > 0 {
                            dp[i][j] = (
                                dp[i][j].0.max(dp[i - 1][j].1 * -grid[i][j] as i64),
                                dp[i][j].1.max(dp[i - 1][j].0 * -grid[i][j] as i64),
                            );
                        }
                        if j > 0 {
                            dp[i][j] = (
                                dp[i][j].0.max(dp[i][j - 1].1 * -grid[i][j] as i64),
                                dp[i][j].1.max(dp[i][j - 1].0 * -grid[i][j] as i64),
                            );
                        }
                    }
                }
            }
        }
        if dp[m - 1][n - 1].1 == -1 {
            dp[m - 1][n - 1].1 as i32
        } else {
            (dp[m - 1][n - 1].1 % 1_000_000_007i64) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_product_path() {
        assert_eq!(
            Solution::max_product_path(vec_vec![[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]]),
            -1
        );
        assert_eq!(
            Solution::max_product_path(vec_vec![[1, -2, 1], [1, -2, 1], [3, -4, 1]]),
            8
        );
        assert_eq!(Solution::max_product_path(vec_vec![[1, 3], [0, -4]]), 0);
    }
}
