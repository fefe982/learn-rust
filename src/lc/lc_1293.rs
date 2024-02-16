// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
// 1293. Shortest Path in a Grid with Obstacles Elimination
pub struct Solution;
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        if k >= m + n - 2 {
            return (m + n - 2) as i32;
        }
        let mut visited = vec![vec![vec![-1; k + 1]; n]; m];
        for i in 0..=k {
            visited[0][0][i] = 0;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 0));
        while let Some((i, j, ik)) = q.pop_front() {
            for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if ni >= m || nj >= n {
                    continue;
                }
                if grid[ni][nj] == 1 && ik >= k {
                    continue;
                }
                if ni == m - 1 && nj == n - 1 {
                    return visited[i][j][ik] + 1;
                }
                let nik = if grid[ni][nj] == 1 { ik + 1 } else { ik };
                if visited[ni][nj][nik] == -1 {
                    for kk in nik..=k {
                        if visited[ni][nj][kk] == -1 {
                            visited[ni][nj][kk] = visited[i][j][ik] + 1;
                        } else {
                            break;
                        }
                    }
                    q.push_back((ni, nj, nik));
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
    fn test_shortest_path() {
        assert_eq!(
            Solution::shortest_path(vec_vec![[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]], 1),
            6
        );
        assert_eq!(
            Solution::shortest_path(vec_vec![[0, 1, 1], [1, 1, 1], [1, 0, 0]], 1),
            -1
        );
    }
}
