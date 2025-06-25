// https://leetcode.com/problems/maximum-rows-covered-by-columns/
// 2397. Maximum Rows Covered by Columns
pub struct Solution;
impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let l = matrix[0].len();
        let matrix = matrix
            .into_iter()
            .map(|row| row.into_iter().enumerate().map(|(i, v)| v << i).sum::<i32>())
            .collect::<Vec<_>>();
        let mut m = 0;
        for i in 0i32..(1 << l) {
            if i.count_ones() != num_select as u32 {
                continue;
            }
            m = m.max(matrix.iter().filter(|&&row| row & i == row).count());
        }
        m as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_rows() {
        assert_eq!(
            Solution::maximum_rows(vec_vec![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 0, 1]], 2),
            3
        );
        assert_eq!(Solution::maximum_rows(vec_vec![[1], [0]], 1), 2);
    }
}
