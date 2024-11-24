// https://leetcode.com/problems/maximum-matrix-sum/
// 1975. Maximum Matrix Sum
pub struct Solution;
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut neg = 0;
        for row in matrix {
            for x in row {
                if x < 0 {
                    neg += 1;
                    min = min.min(-x);
                    sum += -x as i64;
                } else {
                    min = min.min(x);
                    sum += x as i64;
                }
            }
        }
        if neg % 2 == 0 {
            sum
        } else {
            sum - 2 * min as i64
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_matrix_sum() {
        assert_eq!(Solution::max_matrix_sum(vec_vec![[1, -1], [-1, 1]]), 4);
        assert_eq!(
            Solution::max_matrix_sum(vec_vec![[1, 2, 3], [-1, -2, -3], [1, 2, 3]]),
            16
        );
    }
}
