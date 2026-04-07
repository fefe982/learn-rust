// https://leetcode.com/problems/xor-after-range-multiplication-queries-i/
// 3653. XOR After Range Multiplication Queries I
pub struct Solution;
use super::lc_3655;
impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        lc_3655::Solution::xor_after_queries(nums, queries)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn xor_after_queries() {
        assert_eq!(Solution::xor_after_queries(vec![1, 1, 1], vec_vec![[0, 2, 1, 4]]), 4);
        assert_eq!(
            Solution::xor_after_queries(vec![2, 3, 1, 5, 4], vec_vec![[1, 4, 2, 3], [0, 2, 1, 2]]),
            31
        );
    }
}
