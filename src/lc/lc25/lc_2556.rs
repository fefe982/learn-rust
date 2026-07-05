// https://leetcode.com/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/
// 2556. Disconnect Path in a Binary Matrix by at Most One Flip
pub struct Solution;
impl Solution {
    fn find_path(grid: &Vec<Vec<i32>>) -> Vec<Vec<(usize, usize)>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut prev = vec![vec![(usize::MAX, usize::MAX); n]; m];
        prev[0][0] = (usize::MAX - 1, usize::MAX - 1);
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            for (di, dj) in [(0, 1), (1, 0)] {
                let ni = i + di;
                let nj = j + dj;
                if ni < m && nj < n && grid[ni][nj] == 1 && prev[ni][nj] == (usize::MAX, usize::MAX) {
                    prev[ni][nj] = (i, j);
                    if ni == m - 1 && nj == n - 1 {
                        break;
                    }
                    q.push_back((ni, nj));
                }
            }
        }
        prev
    }
    fn fill_grid(grid: &mut Vec<Vec<i32>>, prev: &Vec<Vec<(usize, usize)>>) {
        let mut i = grid.len() - 1;
        let mut j = grid[0].len() - 1;
        if prev[i][j] == (usize::MAX, usize::MAX) {
            return;
        }
        loop {
            let (pi, pj) = prev[i][j];
            if pi == 0 && pj == 0 {
                break;
            }
            grid[pi][pj] = 0;
            i = pi;
            j = pj;
        }
    }
    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
        if grid.len() == 1 && grid[0].len() == 1 {
            return false;
        }
        let mut prev = Solution::find_path(&grid);
        if prev[grid.len() - 1][grid[0].len() - 1] == (usize::MAX, usize::MAX) {
            return true;
        }
        let mut grid = grid;
        Solution::fill_grid(&mut grid, &prev);
        prev = Solution::find_path(&grid);
        prev[grid.len() - 1][grid[0].len() - 1] == (usize::MAX, usize::MAX)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_possible_to_cut_path() {
        assert_eq!(
            Solution::is_possible_to_cut_path(vec_vec![
                [1, 1, 1, 1, 1, 1],
                [1, 0, 1, 1, 1, 1],
                [1, 1, 1, 0, 1, 1],
                [0, 0, 0, 1, 1, 1]
            ]),
            true
        );
        assert_eq!(Solution::is_possible_to_cut_path(vec_vec![[1]]), false);
        assert_eq!(
            Solution::is_possible_to_cut_path(vec_vec![[1, 1, 1], [1, 0, 0], [1, 1, 1]]),
            true
        );
        assert_eq!(
            Solution::is_possible_to_cut_path(vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]),
            false
        );
    }
}
