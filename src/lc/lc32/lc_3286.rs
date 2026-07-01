// https://leetcode.com/problems/find-a-safe-walk-through-a-grid/
// 3286. Find a Safe Walk Through a Grid
pub struct Solution;
impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        if health == grid[0][0] {
            return false;
        }
        let mut visited = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        let mut q = std::collections::BinaryHeap::new();
        q.push((health - grid[0][0], 0, 0));
        visited[0][0] = health - grid[0][0];
        while !q.is_empty() {
            let (h, x, y) = q.pop().unwrap();
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                    let nh = h - grid[nx as usize][ny as usize];
                    if nh > 0 && visited[nx as usize][ny as usize] > nh {
                        if nx == grid.len() as i32 - 1 && ny == grid[0].len() as i32 - 1 {
                            return true;
                        }
                        visited[nx as usize][ny as usize] = nh;
                        q.push((nh, nx as usize, ny as usize));
                    }
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_safe_walk() {
        assert_eq!(
            Solution::find_safe_walk(vec_vec![[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]], 1),
            true
        );
        assert_eq!(
            Solution::find_safe_walk(
                vec_vec![
                    [0, 1, 1, 0, 0, 0],
                    [1, 0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0, 1],
                    [0, 0, 1, 0, 1, 0]
                ],
                3
            ),
            false
        );
        assert_eq!(
            Solution::find_safe_walk(vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]], 5),
            true
        );
    }
}
