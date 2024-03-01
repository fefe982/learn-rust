// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
// 1368. Minimum Cost to Make at Least One Valid Path in a Grid
pub struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut q = std::collections::BinaryHeap::<std::cmp::Reverse<(i32, usize, usize)>>::new();
        q.push(std::cmp::Reverse((0, 0, 0)));
        while let Some(std::cmp::Reverse((cost, x, y))) = q.pop() {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            if x == m - 1 && y == n - 1 {
                return cost;
            }
            for (dir, dx, dy) in [(4, -1, 0), (3, 1, 0), (2, 0, -1), (1, 0, 1)] {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < m && ny < n && !visited[nx][ny] {
                    q.push(std::cmp::Reverse((
                        cost + if grid[x][y] == dir { 0 } else { 1 },
                        nx,
                        ny,
                    )));
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
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(vec_vec![[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]]),
            3
        );
        assert_eq!(Solution::min_cost(vec_vec![[1, 1, 3], [3, 2, 2], [1, 1, 4]]), 0);
        assert_eq!(Solution::min_cost(vec_vec![[1, 2], [4, 3]]), 1);
    }
}
