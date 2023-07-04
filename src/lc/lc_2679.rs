// https://leetcode.com/problems/sum-in-a-matrix/description/
// 2679. Sum in a Matrix
pub struct Solution;
impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|n| n.sort_unstable());
        let mut sum = 0;
        for i in (0..nums[0].len()).rev() {
            sum += nums.iter().map(|n| n[i]).max().unwrap();
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn matrix_sum() {
        assert_eq!(
            Solution::matrix_sum(vec_vec![[7, 2, 1], [6, 4, 2], [6, 5, 3], [3, 2, 1]]),
            15
        );
        assert_eq!(Solution::matrix_sum(vec_vec![[1]]), 1);
    }
}
