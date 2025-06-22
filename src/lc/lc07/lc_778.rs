// https://leetcode.com/problems/swim-in-rising-water/
// 778. Swim in Rising Water
pub struct Solution;
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(grid[0][0]), 0, 0));
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return 0;
        }
        let mut visited = vec![vec![false; n]; m];
        while let Some((h, i, j)) = q.pop() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = (i as i32 + dx) as usize;
                let ny = (j as i32 + dy) as usize;
                if nx < m && ny < n && !visited[nx][ny] {
                    if nx == m - 1 && ny == n - 1 {
                        return grid[nx][ny].max(h.0);
                    }
                    q.push((std::cmp::Reverse(grid[nx][ny].max(h.0)), nx, ny));
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
    fn test_swim_in_rising_water() {
        assert_eq!(Solution::swim_in_water(vec_vec![[0, 2], [1, 3]]), 3);
        assert_eq!(
            Solution::swim_in_water(vec_vec![
                [0, 1, 2, 3, 4],
                [24, 23, 22, 21, 5],
                [12, 13, 14, 15, 16],
                [11, 17, 18, 19, 20],
                [10, 9, 8, 7, 6]
            ]),
            16
        );
    }
}
