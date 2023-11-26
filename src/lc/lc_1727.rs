// https://leetcode.com/problems/largest-submatrix-with-rearrangements/
// 1727. Largest Submatrix With Rearrangements
pub struct Solution;
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let w = matrix[0].len();
        let mut hv = vec![0; w];
        let mut max = 0;
        for row in matrix {
            for (i, v) in row.into_iter().enumerate() {
                if v == 1 {
                    hv[i] += 1;
                } else {
                    hv[i] = 0;
                }
            }
            let mut hvs = hv.clone();
            hvs.sort();
            for (i, v) in hvs.into_iter().enumerate() {
                max = max.max(v * (w - i) as i32);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_largest_submatrix() {
        assert_eq!(
            Solution::largest_submatrix(vec_vec![[0, 0, 1], [1, 1, 1], [1, 0, 1]]),
            4
        );
        assert_eq!(Solution::largest_submatrix(vec_vec![[1, 0, 1, 0, 1]]), 3);
        assert_eq!(Solution::largest_submatrix(vec_vec![[1, 1, 0], [1, 0, 1]]), 2);
    }
}
