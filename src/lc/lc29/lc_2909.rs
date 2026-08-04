// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii/
// 2909. Minimum Sum of Mountain Triplets II
pub struct Solution;
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        super::lc_2908::Solution::minimum_sum(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
