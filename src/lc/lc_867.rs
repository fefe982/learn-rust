// https://leetcode.com/problems/transpose-matrix/
// 867. Transpose Matrix
pub struct Solution;
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut r = vec![vec![0; m]; n];
        for i in 0..m {
            for j in 0..n {
                r[j][i] = matrix[i][j];
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_transpose() {
        assert_eq!(
            Solution::transpose(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec_vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        );
        assert_eq!(
            Solution::transpose(vec_vec![[1, 2, 3], [4, 5, 6]]),
            vec_vec![[1, 4], [2, 5], [3, 6]]
        );
    }
}
