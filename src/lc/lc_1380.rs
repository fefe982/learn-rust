// https://leetcode.com/problems/lucky-numbers-in-a-matrix/
// 1380. Lucky Numbers in a Matrix
pub struct Solution;
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let nrow = matrix.len();
        let ncol = matrix[0].len();
        let mut max = vec![(i32::MIN, 0); ncol];
        let mut min = vec![(i32::MAX, 0); nrow];
        for irow in 0..matrix.len() {
            for icol in 0..matrix[irow].len() {
                if matrix[irow][icol] > max[icol].0 {
                    max[icol] = (matrix[irow][icol], irow);
                }
                if matrix[irow][icol] < min[irow].0 {
                    min[irow] = (matrix[irow][icol], icol);
                }
            }
        }
        let mut ret = vec![];
        for irow in 0..nrow {
            if irow == max[min[irow].1].1 {
                ret.push(min[irow].0)
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_lucky_number() {
        assert_eq!(
            Solution::lucky_numbers(vec_vec![[3, 7, 8], [9, 11, 13], [15, 16, 17]]),
            [15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec_vec![[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]]),
            [12]
        );
        assert_eq!(Solution::lucky_numbers(vec_vec![[7, 8], [1, 2]]), [7]);
    }
}
