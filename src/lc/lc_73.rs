// https://leetcode.com/problems/set-matrix-zeroes/
// 73. Set Matrix Zeroes
pub struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col = false;
        let mut row = false;
        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                row = true;
                break;
            }
        }
        for j in 0..matrix[0].len() {
            if matrix[0][j] == 0 {
                col = true;
                break;
            }
        }
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if row {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
        if col {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(matrix: &mut Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        Solution::set_zeroes(matrix);
        assert_eq!(matrix, &expect);
    }
    #[test]
    fn test_set_zeroes() {
        check(
            &mut vec_vec![[1, 2, 3, 4], [5, 0, 7, 8], [0, 10, 11, 12], [13, 14, 15, 0]],
            vec_vec![[0, 0, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        );
        check(
            &mut vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]],
            vec_vec![[1, 0, 1], [0, 0, 0], [1, 0, 1]],
        );
        check(
            &mut vec_vec![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]],
            vec_vec![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]],
        )
    }
}
