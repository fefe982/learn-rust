// https://leetcode.com/problems/diagonal-traverse/
// 498. Diagonal Traverse
pub struct Solution;
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(mat.len() * mat[0].len());
        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m + n - 1 {
            if i % 2 == 0 {
                for j in (i - i.min(n - 1)..=i.min(m - 1)).rev() {
                    res.push(mat[j][i - j]);
                }
            } else {
                for j in i - i.min(n - 1)..=i.min(m - 1) {
                    res.push(mat[j][i - j]);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_diagonal_order() {
        assert_eq!(
            Solution::find_diagonal_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            [1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
        assert_eq!(Solution::find_diagonal_order(vec_vec![[1, 2], [3, 4]]), [1, 2, 3, 4]);
    }
}
