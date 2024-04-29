// https://leetcode.com/problems/sort-the-matrix-diagonally
// 1329. Sort the Matrix Diagonally
pub struct Solution;
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        for i in 0..mat.len() {
            let mut v = vec![];
            for j in 0..mat[0].len() {
                if i + j < mat.len() {
                    v.push(mat[i + j][j]);
                } else {
                    break;
                }
            }
            v.sort_unstable();
            for j in 0..mat[0].len() {
                if i + j < mat.len() {
                    mat[i + j][j] = v[j];
                } else {
                    break;
                }
            }
        }
        for i in 1..mat[0].len() {
            let mut v = vec![];
            for j in 0..mat.len() {
                if i + j < mat[0].len() {
                    v.push(mat[j][i + j]);
                } else {
                    break;
                }
            }
            v.sort_unstable();
            for j in 0..mat.len() {
                if i + j < mat[0].len() {
                    mat[j][i + j] = v[j];
                } else {
                    break;
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
    fn test_diagonal_sort() {
        assert_eq!(
            Solution::diagonal_sort(vec_vec![[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]]),
            [[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]]
        );
        assert_eq!(
            Solution::diagonal_sort(vec_vec![
                [11, 25, 66, 1, 69, 7],
                [23, 55, 17, 45, 15, 52],
                [75, 31, 36, 44, 58, 8],
                [22, 27, 33, 25, 68, 4],
                [84, 28, 14, 11, 5, 50]
            ]),
            [
                [5, 17, 4, 1, 52, 7],
                [11, 11, 25, 45, 8, 69],
                [14, 23, 25, 44, 58, 15],
                [22, 27, 31, 36, 50, 66],
                [84, 28, 75, 33, 55, 68]
            ]
        );
    }
}
