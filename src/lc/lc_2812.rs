// https://leetcode.com/problems/find-the-safest-path-in-a-grid/
// 2812. Find the Safest Path in a Grid
pub struct Solution;
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return 0;
        }
        let mut grid = grid;
        let mut q = std::collections::VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    q.push_back((i, j));
                } else {
                    grid[i][j] = i32::MAX;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (ni, nj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                if ni >= m || nj >= n || grid[ni][nj] <= grid[i][j] + 1 {
                    continue;
                }
                grid[ni][nj] = grid[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
        let mut score = vec![vec![i32::MAX; n]; m];
        score[0][0] = grid[0][0];
        let mut hp = std::collections::BinaryHeap::new();
        hp.push((score[0][0], 0, 0));
        while let Some((s, i, j)) = hp.pop() {
            if i == m - 1 && j == n - 1 {
                return s;
            }
            for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (ni, nj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                if ni >= m || nj >= n || score[ni][nj] < i32::MAX {
                    continue;
                }
                score[ni][nj] = score[i][j].min(grid[ni][nj]);
                hp.push((score[ni][nj], ni, nj));
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
    fn test_maximum_safeness_factor() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec_vec![[1, 0, 0], [0, 0, 0], [0, 0, 1]]),
            0
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec_vec![[0, 0, 1], [0, 0, 0], [0, 0, 0]]),
            2
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec_vec![[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0]]),
            2
        );
    }
}
