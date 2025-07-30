// https://leetcode.cn/problems/row-with-maximum-ones/
// 2643. Row With Maximum Ones
pub struct Solution;
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut irow = 0;
        let mut imax = 0;
        for (i, row) in mat.into_iter().enumerate() {
            let mut c = 0;
            for x in row {
                if x == 1 {
                    c += 1;
                }
            }
            if c > imax {
                imax = c;
                irow = i as i32;
            }
        }
        vec![irow, imax]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_row_and_maximum_ones() {
        assert_eq!(Solution::row_and_maximum_ones(vec_vec![[0, 1], [1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::row_and_maximum_ones(vec_vec![[0, 0, 0], [0, 1, 1]]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec_vec![[0, 0], [1, 1], [0, 0]]),
            vec![1, 2]
        );
    }
}
