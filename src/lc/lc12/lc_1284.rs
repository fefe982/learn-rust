// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
// 1284. Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
pub struct Solution;
impl Solution {
    fn flip(s: i32, m: i32, n: i32, i: i32, j: i32) -> i32 {
        let mut mask = 1 << (i * n + j);
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i + di;
            let nj = j + dj;
            if ni >= 0 && ni < m && nj >= 0 && nj < n {
                mask |= 1 << (ni * n + nj);
            }
        }
        s ^ mask
    }
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;
        let mut s = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if mat[i][j] == 1 {
                    s |= 1 << (i as i32 * n + j as i32);
                }
            }
        }
        if s == 0 {
            return 0;
        }
        let mut visited = vec![-1; 1 << (m * n)];
        visited[s as usize] = 0;
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(cs) = q.pop_front() {
            for i in 0..m {
                for j in 0..n {
                    let ns = Solution::flip(cs, m, n, i as i32, j as i32);
                    if visited[ns as usize] == -1 {
                        visited[ns as usize] = visited[cs as usize] + 1;
                        q.push_back(ns);
                        if ns == 0 {
                            return visited[ns as usize];
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
    fn test_min_flips() {
        assert_eq!(Solution::min_flips(vec_vec![[0, 0], [0, 1]]), 3);
        assert_eq!(Solution::min_flips(vec_vec![[0]]), 0);
        assert_eq!(Solution::min_flips(vec_vec![[1, 0, 0], [1, 0, 0]]), -1);
    }
}
