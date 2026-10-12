// https://leetcode.com/problems/grid-teleportation-traversal/
// 3552. Grid Teleportation Traversal
pub struct Solution;
impl Solution {
    pub fn min_moves(matrix: Vec<String>) -> i32 {
        let ni = matrix.len();
        let nj = matrix[0].len();
        let mut portal = vec![vec![]; 26];
        let matrix = matrix.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        if matrix[ni - 1][nj - 1] == b'#' {
            return -1;
        }
        if ni == 1 && nj == 1 {
            return 0;
        }
        for i in 0..ni {
            for j in 0..nj {
                if matrix[i][j] >= b'A' && matrix[i][j] <= b'Z' {
                    portal[(matrix[i][j] - b'A') as usize].push((i, j));
                }
            }
        }
        let mut visited = vec![vec![false; nj]; ni];
        visited[0][0] = true;
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 0));
        if matrix[0][0] >= b'A' && matrix[0][0] <= b'Z' {
            for &(i, j) in &portal[(matrix[0][0] - b'A') as usize] {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == ni - 1 && j == nj - 1 {
                    return 0;
                }
                visited[i][j] = true;
                q.push_back((i, j, 0));
            }
        }
        while !q.is_empty() {
            let (i, j, step) = q.pop_front().unwrap();
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (ii, jj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                if ii < ni && jj < nj && !visited[ii][jj] && matrix[ii][jj] != b'#' {
                    visited[ii][jj] = true;
                    q.push_back((ii, jj, step + 1));
                    if ii == ni - 1 && jj == nj - 1 {
                        return step + 1;
                    }
                    if matrix[ii][jj] >= b'A' && matrix[ii][jj] <= b'Z' {
                        for &(x, y) in &portal[(matrix[ii][jj] - b'A') as usize] {
                            if x == ii && y == jj {
                                continue;
                            }
                            if x == ni - 1 && y == nj - 1 {
                                return step + 1;
                            }
                            if !visited[x][y] {
                                visited[x][y] = true;
                                q.push_back((x, y, step + 1));
                            }
                        }
                    }
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
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec_str!["A..", ".A.", "..."]), 2);
        assert_eq!(Solution::min_moves(vec_str![".#...", ".#.#.", ".#.#.", "...#."]), 13);
    }
}
