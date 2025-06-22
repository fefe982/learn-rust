// https://leetcode.com/problems/01-matrix/
// 542. 01 Matrix
pub struct Solution;
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let mut q = std::collections::VecDeque::<(usize, usize)>::new();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 0 {
                    q.push_back((i, j));
                } else {
                    mat[i][j] = -1;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            let dist = mat[i][j] + 1;
            for dir in [[0, -1], [-1, 0], [0, 1], [1, 0]] {
                let ni = (i as i32 + dir[0]) as usize;
                let nj = (j as i32 + dir[1]) as usize;
                if ni < mat.len() && nj < mat[ni].len() && mat[ni][nj] < 0 {
                    q.push_back((ni, nj));
                    mat[ni][nj] = dist;
                }
            }
        }
        mat
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn update_matrix() {
        assert_eq!(
            Solution::update_matrix(vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
            vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]
        );
        assert_eq!(
            Solution::update_matrix(vec_vec![[0, 0, 0], [0, 1, 0], [1, 1, 1]]),
            vec_vec![[0, 0, 0], [0, 1, 0], [1, 2, 1]]
        );
    }
}
