// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
// 2658. Maximum Number of Fish in a Grid
pub struct Solution;
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut fish = grid[i][j];
                if fish == 0 {
                    continue;
                }
                grid[i][j] = 0;
                let mut q = vec![(i, j)];
                while let Some((i, j)) = q.pop() {
                    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (ni, nj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                        if ni < grid.len() && nj < grid[ni].len() && grid[ni][nj] > 0 {
                            fish += grid[ni][nj];
                            grid[ni][nj] = 0;
                            q.push((ni, nj));
                        }
                    }
                }
                max = max.max(fish);
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
    fn test_find_max_fish() {
        assert_eq!(
            Solution::find_max_fish(vec_vec![[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]]),
            7
        );
        assert_eq!(
            Solution::find_max_fish(vec_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]),
            1
        );
    }
}
