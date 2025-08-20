// https://leetcode.com/problems/count-submatrices-with-all-ones/
// 1504. Count Submatrices With All Ones
pub struct Solution;
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut mat = mat;
        let mut sum = 0;
        for i in 0..mat.len() {
            let mut sline = 0;
            let mut vec: Vec<(i32, i32)> = vec![];
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    sline = 0;
                    vec.clear();
                } else {
                    if i > 0 {
                        mat[i][j] = mat[i - 1][j] + 1;
                    }
                    let mut l = 1;
                    while vec.len() > 0 && vec[vec.len() - 1].0 > mat[i][j] {
                        sline -= (vec[vec.len() - 1].0 - mat[i][j]) * vec[vec.len() - 1].1;
                        l += vec[vec.len() - 1].1;
                        vec.pop();
                    }
                    vec.push((mat[i][j], l));
                    sline += mat[i][j];
                    sum += sline;
                }
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_submat() {
        assert_eq!(
            Solution::num_submat(vec_vec![
                [1, 0, 1, 1, 1, 1, 1],
                [1, 1, 0, 0, 0, 1, 1],
                [1, 1, 1, 0, 0, 1, 1],
                [1, 0, 1, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 1, 0, 1, 1, 1, 1],
                [1, 0, 0, 1, 1, 0, 1]
            ]),
            126
        );
        assert_eq!(
            Solution::num_submat(vec_vec![
                [1, 1, 1, 1, 0, 1, 0],
                [1, 1, 1, 0, 0, 0, 1],
                [0, 1, 1, 1, 1, 0, 0],
                [1, 1, 0, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 1, 0, 1, 1, 1, 1],
                [1, 1, 0, 0, 1, 1, 1]
            ]),
            96
        );
        assert_eq!(Solution::num_submat(vec_vec![[1, 0, 1], [1, 1, 0], [1, 1, 0]]), 13);
        assert_eq!(
            Solution::num_submat(vec_vec![[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]]),
            24
        );
    }
}
