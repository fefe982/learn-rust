// https://leetcode.com/problems/minimum-path-cost-in-a-grid/
// 2304. Minimum Path Cost in a Grid
pub struct Solution;
impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let nrow = grid.len();
        let ncol = grid[0].len();
        let mut dp = vec![vec![0; ncol]; nrow];
        for icol in 0..ncol {
            dp[0][icol] = grid[0][icol];
        }
        for irow in 1..nrow {
            for icol in 0..ncol {
                let mut min = i32::MAX;
                for jcol in 0..ncol {
                    min = min.min(dp[irow - 1][jcol] + move_cost[grid[irow - 1][jcol] as usize][icol]);
                }
                dp[irow][icol] = grid[irow][icol] + min;
            }
        }
        let mut min = i32::MAX;
        for icol in 0..ncol {
            min = min.min(dp[nrow - 1][icol])
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_path_cost() {
        assert_eq!(
            Solution::min_path_cost(
                vec_vec![[5, 3], [4, 0], [2, 1]],
                vec_vec![[9, 8], [1, 5], [10, 12], [18, 6], [2, 4], [14, 3]]
            ),
            17
        );
        assert_eq!(
            Solution::min_path_cost(
                vec_vec![[5, 1, 2], [4, 0, 3]],
                vec_vec![
                    [12, 10, 15],
                    [20, 23, 8],
                    [21, 7, 1],
                    [8, 1, 13],
                    [9, 10, 25],
                    [5, 3, 2]
                ]
            ),
            6
        );
    }
}
