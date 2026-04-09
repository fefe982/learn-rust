// https://leetcode.com/problems/minimum-distance-between-three-equal-elements-i/
// 3740. Minimum Distance Between Three Equal Elements I
pub struct Solution;
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        super::lc_3741::Solution::minimum_distance(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_distance() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
