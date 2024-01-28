// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
// 1074. Number of Submatrices That Sum to Target
pub struct Solution;
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut sum = vec![vec![0; n]; m];
        let mut cnt = 0;
        for i in 0..m {
            for k in 0..=i {
                let mut freq = std::collections::HashMap::<i32, i32>::new();
                let mut s = 0;
                freq.insert(0, 1);
                for j in 0..n {
                    sum[k][j] += matrix[i][j];
                    s += sum[k][j];
                    cnt += freq.get(&(s - target)).unwrap_or(&0);
                    *freq.entry(s).or_default() += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_submatrix_sum_target() {
        assert_eq!(
            Solution::num_submatrix_sum_target(vec_vec![[904, 1, 0], [1, 1, 1], [0, 1, 0]], 0),
            3
        );
        assert_eq!(
            Solution::num_submatrix_sum_target(vec_vec![[0, 1, 0], [1, 1, 1], [0, 1, 0]], 0),
            4
        );
        assert_eq!(Solution::num_submatrix_sum_target(vec_vec![[1, -1], [-1, 1]], 0), 5);
        assert_eq!(Solution::num_submatrix_sum_target(vec_vec![[904]], 0), 0);
    }
}
