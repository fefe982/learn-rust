// https://leetcode.com/problems/k-highest-ranked-items-within-a-price-range/
// 2146. K Highest Ranked Items Within a Price Range
pub struct Solution;
impl Solution {
    pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        q.push_back((0, start[0] as usize, start[1] as usize));
        let mut res = Vec::new();
        let mut last_d = 0;
        while let Some((d, i, j)) = q.pop_front() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            if grid[i][j] >= pricing[0] && grid[i][j] <= pricing[1] {
                if res.len() >= k as usize && d > last_d {
                    break;
                }
                res.push((d, grid[i][j], i, j));
                last_d = d;
            }
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0
                    && ni < grid.len() as i32
                    && nj >= 0
                    && nj < grid[0].len() as i32
                    && !visited[ni as usize][nj as usize]
                    && grid[ni as usize][nj as usize] != 0
                {
                    q.push_back((d + 1, ni as usize, nj as usize));
                }
            }
        }
        res.sort_unstable();
        res.into_iter()
            .take(k as usize)
            .map(|(_, _, i, j)| vec![i as i32, j as i32])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_highest_ranked_k_items() {
        assert_eq!(
            Solution::highest_ranked_k_items(
                vec_vec![[1, 2, 0, 1], [1, 3, 0, 1], [0, 2, 5, 1]],
                vec![2, 5],
                vec![0, 0],
                3
            ),
            vec_vec![[0, 1], [1, 1], [2, 1]]
        );
        assert_eq!(
            Solution::highest_ranked_k_items(
                vec_vec![[1, 2, 0, 1], [1, 3, 3, 1], [0, 2, 5, 1]],
                vec![2, 3],
                vec![2, 3],
                2
            ),
            vec_vec![[2, 1], [1, 2]]
        );
        assert_eq!(
            Solution::highest_ranked_k_items(vec_vec![[1, 1, 1], [0, 0, 1], [2, 3, 4]], vec![2, 3], vec![0, 0], 3),
            vec_vec![[2, 1], [2, 0]]
        );
    }
}
