// https://leetcode.com/problems/minimum-path-sum/description/
// 64. Minimum Path Sum
pub struct Solution;
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 1..grid[0].len() {
            grid[0][i] += grid[0][i - 1];
        }
        for i in 1..grid.len() {
            grid[i][0] += grid[i - 1][0];
            for j in 1..grid[i].len() {
                grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1]);
            }
        }
        grid[grid.len() - 1][grid[0].len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
