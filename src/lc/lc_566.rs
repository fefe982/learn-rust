// https://leetcode.cn/problems/reshape-the-matrix/
// 566. Reshape the Matrix
pub struct Solution;
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let m = mat.len();
        let n = mat[0].len();
        if m * n != r * c || r == m {
            return mat;
        }
        let mut ret = vec![vec![0; c]; r];
        for i in 0..m {
            for j in 0..n {
                ret[(i * n + j) / c][(i * n + j) % c] = mat[i][j];
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn matrix_reshape() {
        assert_eq!(
            Solution::matrix_reshape(vec_vec![[1, 2], [3, 4]], 1, 4),
            vec_vec![[1, 2, 3, 4]]
        );
        assert_eq!(
            Solution::matrix_reshape(vec_vec![[1, 2], [3, 4]], 2, 4),
            vec_vec![[1, 2], [3, 4]]
        )
    }
}
