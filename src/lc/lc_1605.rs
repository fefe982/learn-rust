// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/
// 1605. Find Valid Matrix Given Row and Column Sums
pub struct Solution;
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i_row in 0..row_sum.len() {
            for i_col in 0..col_sum.len() {
                let val = std::cmp::min(col_sum[i_col], row_sum[i_row]);
                res[i_row][i_col] = val;
                col_sum[i_col] -= val;
                row_sum[i_row] -= val;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restore_matrix() {
        assert_eq!(
            Solution::restore_matrix(vec![3, 8], vec![4, 7]),
            vec![vec![3, 0], vec![1, 7]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
            vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]]
        );
    }
}
