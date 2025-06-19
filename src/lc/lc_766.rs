// https://leetcode.com/problems/toeplitz-matrix/
// 766. Toeplitz Matrix
pub struct Solution;
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 1..matrix[0].len() {
            let e = matrix[0][i - 1];
            for j in 1..matrix.len().min(matrix[0].len() + 1 - i) {
                if matrix[j][i + j - 1] != e {
                    return false;
                }
            }
        }
        for i in 2..matrix.len() {
            let e = matrix[i - 1][0];
            for j in 1..matrix[0].len().min(matrix.len() + 1 - i) {
                if matrix[i + j - 1][j] != e {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_toeplitz_matrix() {
        assert_eq!(
            Solution::is_toeplitz_matrix(vec_vec![[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]]),
            true
        );
        assert_eq!(Solution::is_toeplitz_matrix(vec_vec![[1, 2], [2, 2]]), false);
    }
}
