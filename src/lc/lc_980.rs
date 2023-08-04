// https://leetcode.com/problems/unique-paths-iii/
// 980. Unique Paths III
pub struct Solution;
impl Solution {
    fn walk(grid: &mut Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize), l: i32) -> i32 {
        let mut cnt = 0;
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (
                (start.0 as i32 + dir.0) as usize,
                (start.1 as i32 + dir.1) as usize,
            );
            if next.0 >= grid.len() || next.1 >= grid[0].len() {
                continue;
            }
            if grid[next.0][next.1] == 0 {
                grid[next.0][next.1] = -1;
                cnt += Self::walk(grid, next, end, l - 1);
                grid[next.0][next.1] = 0;
            } else if grid[next.0][next.1] == 2 && l == 0 {
                cnt += 1;
            }
        }
        cnt
    }
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut l = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                match grid[i][j] {
                    1 => start = (i, j),
                    2 => end = (i, j),
                    0 => l += 1,
                    -1 => (),
                    _ => unreachable!(),
                }
            }
        }
        Self::walk(&mut grid, start, end, l)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn unique_paths_iii() {
        assert_eq!(
            Solution::unique_paths_iii(vec_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_iii(vec_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]),
            4
        );
        assert_eq!(Solution::unique_paths_iii(vec_vec![[0, 1], [2, 0]]), 0);
    }
}
