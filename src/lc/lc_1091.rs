// https://leetcode.com/problems/shortest-path-in-binary-matrix/
// 1091. Shortest Path in Binary Matrix
pub struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let l0 = grid.len();
        let l1 = grid[0].len();
        if grid[0][0] != 0 || grid[l0 - 1][l1 - 1] != 0 {
            return -1;
        }
        if l0 == 1 && l1 == 1 {
            return 1;
        }
        grid[0][0] = -1;
        let mut q = std::collections::VecDeque::<(usize, usize)>::new();
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            let s = grid[i][j];
            for di in if i == 0 { 0 } else { -1 }..=if i + 1 == l0 { 0 } else { 1 } {
                let ni = (i as i32 + di) as usize;
                for dj in if j == 0 { 0 } else { -1 }..=if j + 1 == l1 { 0 } else { 1 } {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let nj = (j as i32 + dj) as usize;
                    if ni == l0 - 1 && nj == l1 - 1 {
                        return 1 - s;
                    }
                    if grid[ni][nj] == 0 {
                        grid[ni][nj] = s - 1;
                        q.push_back((ni, nj));
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
    fn shortest_path_binary_matrix() {
        assert_eq!(Solution::shortest_path_binary_matrix(vec_vec![[0]]), 1);
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec_vec![[0, 1], [1, 0]]),
            2
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec_vec![[0, 0, 0], [1, 1, 0], [1, 1, 0]]),
            4
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec_vec![[1, 0, 0], [1, 1, 0], [1, 1, 0]]),
            -1
        );
    }
}
