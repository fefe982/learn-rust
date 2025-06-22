// https://leetcode.com/problems/rotate-image/
// 48. Rotate Image
pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len / 2 {
            for j in i..len - i - 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[len - j - 1][i];
                matrix[len - j - 1][i] = matrix[len - i - 1][len - j - 1];
                matrix[len - i - 1][len - j - 1] = matrix[j][len - i - 1];
                matrix[j][len - i - 1] = tmp;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(mut matrix: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expect);
    }
    #[test]
    fn rotate() {
        check(
            vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            vec_vec![[7, 4, 1], [8, 5, 2], [9, 6, 3]],
        );
        check(
            vec_vec![[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]],
            vec_vec![[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]],
        );
    }
}
