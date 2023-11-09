// https://leetcode.com/problems/escape-the-spreading-fire/
// 2258. Escape the Spreading Fire
pub struct Solution;
impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut fire = vec![vec![-1; m]; n];
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    q.push_back((0, i, j));
                    fire[i][j] = 0;
                }
            }
        }
        while let Some((t, i, j)) = q.pop_front() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;
                if x < n && y < m && grid[x][y] == 0 && fire[x][y] == -1 {
                    fire[x][y] = t + 1;
                    q.push_back((t + 1, x, y));
                }
            }
        }
        if fire[0][0] == -1 {
            if fire[n - 1][m - 1] != -1 {
                return -1;
            }
            q.push_back((0, 0, 0));
            while let Some((t, i, j)) = q.pop_front() {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let x = (i as i32 + dx) as usize;
                    let y = (j as i32 + dy) as usize;
                    if x < n && y < m && grid[x][y] == 0 && fire[x][y] == -1 {
                        fire[x][y] = t + 1;
                        q.push_back((t + 1, x, y));
                    }
                }
            }
            if fire[n - 1][m - 1] == -1 {
                return -1;
            }
            return 10_0000_0000;
        }
        let mut h = std::collections::BinaryHeap::new();
        h.push((fire[0][0], 0, 0));
        let mut grid = grid;
        grid[0][0] = 2;
        while let Some((t, i, j)) = h.pop() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;
                if x < n && y < m && grid[x][y] == 0 {
                    let ndiff = t - 1 + fire[x][y] - fire[i][j];
                    if ndiff >= 0 && x == n - 1 && y == m - 1 {
                        return if ndiff < t { ndiff } else { ndiff - 1 };
                    }
                    if ndiff > 0 {
                        grid[x][y] = 2;
                        h.push((ndiff, x, y));
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_minutes() {
        assert_eq!(
            Solution::maximum_minutes(vec_vec![
                [0, 0, 2, 2, 1, 1, 0, 2, 1, 1, 2, 2, 0, 2, 2, 1, 2, 0, 1, 2, 2, 0, 1, 2, 2, 1, 2, 2],
                [2, 2, 2, 1, 1, 2, 2, 1, 2, 0, 1, 1, 1, 2, 2, 1, 1, 0, 2, 2, 2, 0, 1, 0, 1, 2, 2, 2],
                [0, 0, 1, 1, 0, 1, 2, 0, 1, 1, 1, 1, 0, 2, 0, 2, 0, 2, 1, 1, 0, 2, 1, 2, 2, 2, 1, 2],
                [2, 2, 0, 0, 0, 0, 1, 0, 1, 0, 2, 0, 1, 0, 2, 0, 0, 1, 2, 1, 0, 1, 1, 1, 2, 0, 2, 0],
                [2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 2, 0, 1, 1, 1, 1, 2, 0, 2, 1, 1, 2, 0, 2, 0, 2, 0],
                [0, 1, 0, 1, 2, 2, 2, 0, 2, 0, 2, 2, 1, 2, 0, 0, 1, 0, 2, 0, 2, 0, 1, 2, 2, 0, 2, 0],
                [1, 0, 2, 2, 2, 0, 2, 0, 2, 0, 2, 0, 1, 0, 2, 2, 0, 2, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0],
                [0, 1, 2, 0, 1, 0, 1, 0, 2, 1, 2, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 2, 0, 1, 0, 1, 0, 2],
                [2, 1, 1, 0, 1, 1, 2, 2, 1, 2, 2, 1, 0, 1, 0, 0, 0, 2, 1, 0, 2, 2, 1, 2, 1, 2, 0, 1],
                [1, 1, 2, 0, 2, 2, 1, 2, 0, 2, 1, 1, 0, 0, 0, 2, 2, 2, 2, 1, 2, 2, 0, 2, 1, 1, 2, 0],
                [2, 1, 2, 2, 0, 0, 1, 0, 1, 2, 1, 0, 1, 0, 2, 0, 0, 1, 1, 0, 2, 0, 2, 0, 1, 2, 2, 0],
                [1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 1, 2, 1, 1, 0, 1, 0, 0, 2, 1, 2, 1, 0, 2],
                [2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 2, 1, 2, 0, 2, 2, 2, 1, 0, 2, 1, 0, 1, 2, 1, 0, 1, 1],
                [0, 2, 2, 1, 0, 2, 1, 0, 1, 2, 2, 1, 2, 2, 1, 2, 0, 1, 2, 2, 0, 2, 1, 0, 2, 1, 0, 0],
                [0, 2, 2, 2, 1, 2, 1, 0, 0, 2, 2, 0, 1, 0, 2, 1, 0, 0, 2, 1, 1, 1, 2, 1, 2, 1, 0, 1],
                [2, 2, 2, 1, 1, 1, 1, 0, 2, 2, 2, 1, 0, 0, 2, 2, 0, 0, 1, 1, 0, 0, 2, 1, 2, 1, 2, 2],
                [2, 1, 2, 1, 1, 1, 0, 2, 1, 0, 1, 1, 2, 1, 0, 0, 1, 1, 2, 1, 2, 2, 1, 2, 0, 2, 0, 0]
            ]),
            -1
        );
        assert_eq!(
            Solution::maximum_minutes(vec_vec![
                [0, 2, 0, 0, 1],
                [0, 2, 0, 2, 2],
                [0, 2, 0, 0, 0],
                [0, 0, 2, 2, 0],
                [0, 0, 0, 0, 0]
            ]),
            0
        );
        assert_eq!(
            Solution::maximum_minutes(vec_vec![
                [0, 2, 0, 0, 0, 0, 0],
                [0, 0, 0, 2, 2, 1, 0],
                [0, 2, 0, 0, 1, 2, 0],
                [0, 0, 2, 2, 2, 0, 2],
                [0, 0, 0, 0, 0, 0, 0]
            ]),
            3
        );
        assert_eq!(
            Solution::maximum_minutes(vec_vec![[0, 0, 0, 0], [0, 1, 2, 0], [0, 2, 0, 0]]),
            -1
        );
        assert_eq!(
            Solution::maximum_minutes(vec_vec![[0, 0, 0], [2, 2, 0], [1, 2, 0]]),
            1000000000
        );
    }
}
