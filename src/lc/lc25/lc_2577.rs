// https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid/
// 2577. Minimum Time to Visit a Cell in a Grid
pub struct Solution;
impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        let mut grid = grid;
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0, 0));
        grid[0][0] = i32::MAX;
        let li = grid.len();
        let lj = grid[0].len();
        while let Some((std::cmp::Reverse(t), i, j)) = q.pop() {
            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if ni >= li || nj >= lj || grid[ni][nj] == i32::MAX {
                    continue;
                }
                if grid[ni][nj] % 2 != (ni + nj) as i32 % 2 {
                    grid[ni][nj] += 1;
                }
                let nt = (t + 1).max(grid[ni][nj]);
                if ni == li - 1 && nj == lj - 1 {
                    return nt;
                }
                q.push((std::cmp::Reverse(nt), ni, nj));
                grid[ni][nj] = i32::MAX;
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
    fn test_minimum_time() {
        assert_eq!(
            Solution::minimum_time(vec_vec![[0, 1, 3, 2], [5, 1, 2, 5], [4, 3, 8, 6]]),
            7
        );
        assert_eq!(Solution::minimum_time(vec_vec![[0, 2, 4], [3, 2, 1], [1, 0, 4]]), -1);
    }
}
