// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/
// 3318. Find X-Sum of All K-Long Subarrays I
pub struct Solution;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        super::lc_3321::Solution::find_x_sum(nums, k, x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_x_sum() {
        assert_eq!(
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
