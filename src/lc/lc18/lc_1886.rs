// https://leetcode.cn/problems/determine-whether-matrix-can-be-obtained-by-rotation/
// 1886. Determine Whether Matrix Can Be Obtained By Rotation
pub struct Solution;
impl Solution {
    pub fn rotate(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        for i in 0..n / 2 {
            for j in 0..(n + 1) / 2 {
                let tmp = mat[i][j];
                mat[i][j] = mat[n - j - 1][i];
                mat[n - j - 1][i] = mat[n - i - 1][n - j - 1];
                mat[n - i - 1][n - j - 1] = mat[j][n - i - 1];
                mat[j][n - i - 1] = tmp;
            }
        }
        mat
    }
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        for _ in 0..4 {
            println!("{:?}", mat);
            if mat == target {
                return true;
            }
            mat = Self::rotate(mat);
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_rotation() {
        assert_eq!(
            Solution::find_rotation(
                vec_vec![[0, 1, 1, 0], [1, 0, 0, 1], [0, 1, 0, 1], [0, 0, 1, 0]],
                vec_vec![[0, 1, 0, 0], [1, 0, 1, 0], [1, 0, 0, 1], [0, 1, 1, 0]]
            ),
            true
        );
        assert_eq!(
            Solution::find_rotation(vec_vec![[0, 1], [1, 0]], vec_vec![[1, 0], [0, 1]]),
            true
        );
        assert_eq!(
            Solution::find_rotation(vec_vec![[0, 1], [1, 1]], vec_vec![[1, 0], [0, 1]]),
            false
        );
        assert_eq!(
            Solution::find_rotation(
                vec_vec![[0, 0, 0], [0, 1, 0], [1, 1, 1]],
                vec_vec![[1, 1, 1], [0, 1, 0], [0, 0, 0]]
            ),
            true
        );
    }
}
