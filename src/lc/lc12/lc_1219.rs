// https://leetcode.com/problems/path-with-maximum-gold/
// 1219. Path with Maximum Gold
pub struct Solution;
impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, gold: i32) -> i32 {
        let mut max = gold;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (i as i32 + dx) as usize;
            let ny = (j as i32 + dy) as usize;
            if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] && grid[nx][ny] != 0 {
                visited[nx][ny] = true;
                max = max.max(Self::dfs(grid, visited, nx, ny, gold + grid[nx][ny]));
                visited[nx][ny] = false;
            }
        }
        max
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    visited[i][j] = true;
                    max = max.max(Self::dfs(&grid, &mut visited, i, j, grid[i][j]));
                    visited[i][j] = false;
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_maximum_gold() {
        assert_eq!(
            Solution::get_maximum_gold(vec_vec![[0, 6, 0], [5, 8, 7], [0, 9, 0]]),
            24
        );
        assert_eq!(
            Solution::get_maximum_gold(vec_vec![[1, 0, 7], [2, 0, 6], [3, 4, 5], [0, 3, 0], [9, 0, 20]]),
            28
        );
    }
}
