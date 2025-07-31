// https://leetcode.com/problems/shortest-bridge/
// 934. Shortest Bridge
pub struct Solution;
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let dir = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        let mut q1 = std::collections::VecDeque::new();
        let mut qn = std::collections::VecDeque::new();
        'find: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q1.push_back((i, j));
                    break 'find;
                }
            }
        }
        while let Some((x, y)) = q1.pop_front() {
            if grid[x][y] != 1 {
                continue;
            }
            grid[x][y] = -1;
            for i in 0..4 {
                let nx = (x as i32 + dir[i][0]) as usize;
                let ny = (y as i32 + dir[i][1]) as usize;
                if nx < n && ny < n {
                    if grid[nx][ny] == 1 {
                        q1.push_back((nx, ny));
                    } else if grid[nx][ny] == 0 {
                        grid[nx][ny] = 2;
                        qn.push_back((nx, ny));
                    }
                }
            }
        }
        while let Some((x, y)) = qn.pop_front() {
            if grid[x][y] < 0 {
                continue;
            }
            let c = grid[x][y];
            grid[x][y] = -1;
            for i in 0..4 {
                let nx = (x as i32 + dir[i][0]) as usize;
                let ny = (y as i32 + dir[i][1]) as usize;
                if nx < n && ny < n {
                    if grid[nx][ny] == 1 {
                        return c - 1;
                    } else if grid[nx][ny] == 0 {
                        grid[nx][ny] = c + 1;
                        qn.push_back((nx, ny));
                    }
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shortest_bridge() {
        assert_eq!(Solution::shortest_bridge(vec_vec![[0, 1], [1, 0]]), 1);
        assert_eq!(Solution::shortest_bridge(vec_vec![[0, 1, 0], [0, 0, 0], [0, 0, 1]]), 2);
        assert_eq!(
            Solution::shortest_bridge(vec_vec![
                [1, 1, 1, 1, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1]
            ]),
            1
        );
    }
}
