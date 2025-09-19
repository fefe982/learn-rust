// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
// 1292. Maximum Side Length of a Square with Sum Less Than or Equal to Threshold
pub struct Solution;
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut res = 0;
        let mut len = 1;
        let mut sum = vec![vec![0; mat[0].len() + 1]; mat.len() + 1];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + mat[i][j];
                if i + 1 >= len && j + 1 >= len {
                    let s = sum[i + 1][j + 1] - sum[i + 1 - len][j + 1] - sum[i + 1][j + 1 - len]
                        + sum[i + 1 - len][j + 1 - len];
                    if s <= threshold {
                        res = len;
                        len += 1;
                    }
                }
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_side_length() {
        assert_eq!(
            Solution::max_side_length(
                vec_vec![[1, 1, 3, 2, 4, 3, 2], [1, 1, 3, 2, 4, 3, 2], [1, 1, 3, 2, 4, 3, 2]],
                4
            ),
            2
        );
        assert_eq!(
            Solution::max_side_length(
                vec_vec![
                    [2, 2, 2, 2, 2],
                    [2, 2, 2, 2, 2],
                    [2, 2, 2, 2, 2],
                    [2, 2, 2, 2, 2],
                    [2, 2, 2, 2, 2]
                ],
                1
            ),
            0
        );
    }
}
