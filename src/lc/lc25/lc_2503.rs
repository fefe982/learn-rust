// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
// 2503. Maximum Number of Points From Grid Queries
pub struct Solution;
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; queries.len()];
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|&(_, v)| v);
        let mut grid = grid;
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(grid[0][0]), 0, 0));
        grid[0][0] = 0;
        let mut cnt = 0;
        for (i, query) in queries {
            while let Some(&(std::cmp::Reverse(v), ci, cj)) = q.peek() {
                if v >= query {
                    break;
                }
                q.pop();
                cnt += 1;
                for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (ni, nj) = ((ci as i32 + di) as usize, (cj as i32 + dj) as usize);
                    if ni < grid.len() && nj < grid[0].len() && grid[ni][nj] > 0 {
                        q.push((std::cmp::Reverse(grid[ni][nj]), ni, nj));
                        grid[ni][nj] = 0;
                    }
                }
            }
            res[i] = cnt;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_points() {
        assert_eq!(
            Solution::max_points(vec_vec![[1, 2, 3], [2, 5, 7], [3, 5, 1]], vec![5, 6, 2]),
            [5, 8, 1]
        );
        assert_eq!(Solution::max_points(vec_vec![[5, 2, 1], [1, 1, 2]], vec![3]), [0]);
    }
}
