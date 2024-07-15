// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/
// 2290. Minimum Obstacle Removal to Reach Corner
pub struct Solution;
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut q0 = vec![(0, 0)];
        let mut q1 = vec![];
        let mut grid = grid;
        let mut r0 = 0;
        grid[0][0] = -1;
        loop {
            while let Some((x, y)) = q0.pop() {
                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                    if nx >= grid.len() || ny >= grid[0].len() || grid[nx][ny] == -1 {
                        continue;
                    }
                    if nx == grid.len() - 1 && ny == grid[0].len() - 1 {
                        return r0;
                    }
                    if grid[nx][ny] == 1 {
                        q1.push((nx, ny));
                    } else {
                        q0.push((nx, ny));
                    }
                    grid[nx][ny] = -1;
                }
            }
            q0 = q1;
            q1 = vec![];
            r0 += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_obstacles() {
        assert_eq!(
            Solution::minimum_obstacles(vec_vec![[0, 1, 1], [1, 1, 0], [1, 1, 0]]),
            2
        );
        assert_eq!(
            Solution::minimum_obstacles(vec_vec![[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]),
            0
        );
    }
}
