// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
// 2435. Paths in Matrix Whose Sum Is Divisible by K
pub struct Solution;
impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = 1_0000_0000_7;
        let nr = grid.len();
        let nc = grid[0].len();
        let k = k as usize;
        let mut dp = vec![vec![vec![0; k]; nc]; nr];
        dp[0][0][grid[0][0] as usize % k] = 1;
        for i in 0..nr {
            for j in 0..nc {
                let v = grid[i][j] as usize;
                for l in 0..k {
                    if i > 0 {
                        dp[i][j][(l + v) % k] = (dp[i][j][(l + v) % k] + dp[i - 1][j][l]) % m;
                    }
                    if j > 0 {
                        dp[i][j][(l + v) % k] = (dp[i][j][(l + v) % k] + dp[i][j - 1][l]) % m
                    }
                }
            }
        }
        dp[nr - 1][nc - 1][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_paths() {
        assert_eq!(
            Solution::number_of_paths(vec_vec![[5, 2, 4], [3, 0, 5], [0, 7, 2]], 3),
            2
        );
        assert_eq!(Solution::number_of_paths(vec_vec![[0, 0]], 5), 1);
        assert_eq!(
            Solution::number_of_paths(vec_vec![[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]], 1),
            10
        );
    }
}
