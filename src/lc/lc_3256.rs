// https://leetcode.com/problems/maximum-value-sum-by-placing-three-rooks-i/
// 3256. Maximum Value Sum by Placing Three Rooks I
pub struct Solution;
impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        super::lc_3257::Solution::maximum_value_sum(board)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_value_sum() {
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[-3, 1, 1, 1], [-3, 1, -3, 1], [-3, 2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            15
        );
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]]),
            3
        );
    }
}
