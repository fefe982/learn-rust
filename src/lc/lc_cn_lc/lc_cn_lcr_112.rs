// https://leetcode.cn/problems/fpTFWP/
// LCR 112. 矩阵中的最长递增路径
pub struct Solution;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        super::super::lc03::lc_329::Solution::longest_increasing_path(matrix)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_increasing_path() {
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[9, 9, 4], [6, 6, 8], [2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[3, 4, 5], [3, 2, 6], [2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec_vec![[1]]), 1);
    }
}
