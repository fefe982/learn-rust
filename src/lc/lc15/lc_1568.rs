// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/
// 1568. Minimum Number of Days to Disconnect Island
pub struct Solution;
impl Solution {
    fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
        let mut ans = 0;
        let mut q = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && !vis[i][j] {
                    ans += 1;
                    q.push((i, j));
                    vis[i][j] = true;
                    while let Some((ci, cj)) = q.pop() {
                        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (ni, nj) = ((ci as i32 + di) as usize, (cj as i32 + dj) as usize);
                            if ni < grid.len()
                                && nj < grid[0].len()
                                && grid[ni][nj] == 1
                                && !vis[ni][nj]
                                && grid[ni][nj] == 1
                            {
                                vis[ni][nj] = true;
                                q.push((ni, nj));
                            }
                        }
                    }
                }
            }
        }
        ans
    }
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = Self::count_islands(&grid);
        if n == 0 || n > 1 {
            return 0;
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if Self::count_islands(&grid) != 1 {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }
        2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_days() {
        assert_eq!(
            Solution::min_days(vec_vec![[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]),
            2
        );
        assert_eq!(Solution::min_days(vec_vec![[1, 1]]), 2);
    }
}
