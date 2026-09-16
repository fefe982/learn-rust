// https://leetcode.com/problems/count-paths-with-the-given-xor-value/
// 3393. Count Paths With the Given XOR Value
pub struct Solution;
impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, cur: i32, k: i32, i: usize, j: usize, dp: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        if i >= grid.len() || j >= grid[0].len() {
            return 0;
        }
        let c = cur ^ grid[i][j];
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return if c == k { 1 } else { 0 };
        }
        if dp[i][j][cur as usize] != -1 {
            return dp[i][j][cur as usize];
        }
        let right = Self::dfs(grid, c, k, i, j + 1, dp) % 1_000_000_007i64;
        let down = Self::dfs(grid, c, k, i + 1, j, dp) % 1_000_000_007i64;
        let sum = (right + down) % 1_000_000_007i64;
        dp[i][j][cur as usize] = sum;
        sum
    }
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; 16]; grid[0].len()]; grid.len()];
        Self::dfs(&grid, 0, k, 0, 0, &mut dp) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_paths_with_xor_value() {
        assert_eq!(
            Solution::count_paths_with_xor_value(vec_vec![[2, 1, 5], [7, 10, 0], [12, 6, 4]], 11),
            3
        );
        assert_eq!(
            Solution::count_paths_with_xor_value(vec_vec![[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], 2),
            5
        );
        assert_eq!(
            Solution::count_paths_with_xor_value(vec_vec![[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], 10),
            0
        );
    }
}
