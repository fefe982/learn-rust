// https://leetcode.com/problems/unique-paths-ii/
// 63. Unique Paths II
pub struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[obstacle_grid.len() - 1][obstacle_grid[obstacle_grid.len() - 1].len() - 1]
            != 0
            || obstacle_grid[0][0] != 0
        {
            return 0;
        }
        obstacle_grid[0][0] = -1;
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[i].len() {
                if obstacle_grid[i][j] != 0 {
                    continue;
                }
                let mut s = 0;
                if i > 0 && obstacle_grid[i - 1][j] < 0 {
                    s += obstacle_grid[i - 1][j];
                }
                if j > 0 && obstacle_grid[i][j - 1] < 0 {
                    s += obstacle_grid[i][j - 1];
                }
                obstacle_grid[i][j] = s;
            }
        }
        -obstacle_grid[obstacle_grid.len() - 1][obstacle_grid[obstacle_grid.len() - 1].len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn unique_paths_with_obstacles() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec_vec![[0, 0], [0, 1]]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec_vec![[0, 1], [0, 0]]),
            1
        );
    }
}
