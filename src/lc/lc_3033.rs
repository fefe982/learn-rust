// https://leetcode.com/problems/modify-the-matrix/
// 3033. Modify the Matrix
pub struct Solution;
impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        let mut max = vec![0; matrix[0].len()];
        for row in &matrix {
            for (i, &v) in row.iter().enumerate() {
                max[i] = max[i].max(v);
            }
        }
        for row in matrix.iter_mut() {
            for (i, v) in row.iter_mut().enumerate() {
                if *v < 0 {
                    *v = max[i];
                }
            }
        }
        matrix
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_modified_matrix() {
        assert_eq!(
            Solution::modified_matrix(vec_vec![[1, 2, -1], [4, -1, 6], [7, 8, 9]]),
            vec_vec![[1, 2, 9], [4, 8, 6], [7, 8, 9]]
        );
        assert_eq!(
            Solution::modified_matrix(vec_vec![[3, -1], [5, 2]]),
            vec_vec![[3, 2], [5, 2]]
        );
    }
}
