// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
// 1072. Flip Columns For Maximum Number of Equal Rows
pub struct Solution;
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut cnt = std::collections::HashMap::<Vec<i32>, i32>::new();
        let mut max = 0;
        for mut v in matrix {
            if v[0] == 1 {
                for n in v.iter_mut() {
                    *n = 1 - *n;
                }
            }
            let c = cnt.entry(v).or_default();
            *c += 1;
            if *c >= max {
                max = *c;
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
    fn max_equal_rows_after_flips() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec_vec![[0, 1], [1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec_vec![[0, 1], [1, 0]]),
            2
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec_vec![[0, 0, 0], [0, 0, 1], [1, 1, 0]]),
            2
        );
    }
}
