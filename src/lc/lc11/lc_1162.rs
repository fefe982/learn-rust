// https://leetcode.com/problems/as-far-from-land-as-possible/
// 1162. As Far from Land as Possible
pub struct Solution;
impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut q = std::collections::VecDeque::<(usize, usize, i32)>::new();
        let mut md = -1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    q.push_back((i, j, 0));
                }
            }
        }
        while let Some((i, j, d)) = q.pop_front() {
            for (ni, nj) in [(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)] {
                if ni < grid.len() && nj < grid[0].len() && grid[ni][nj] == 0 {
                    grid[ni][nj] = 1;
                    q.push_back((ni, nj, d + 1));
                    md = md.max(d + 1);
                }
            }
        }
        md
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_distance() {
        assert_eq!(Solution::max_distance(vec_vec![[1, 0, 1], [0, 0, 0], [1, 0, 1]]), 2);
        assert_eq!(Solution::max_distance(vec_vec![[1, 0, 0], [0, 0, 0], [0, 0, 0]]), 4);
    }
}
