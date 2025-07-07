// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
// 1749. Maximum Absolute Sum of Any Subarray
pub struct Solution;
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut rmin = 0;
        let mut rmax = 0;
        let mut min = 0;
        let mut max = 0;
        for n in nums {
            rmin = (rmin + n).min(0);
            rmax = (rmax + n).max(0);
            min = min.min(rmin);
            max = max.max(rmax);
        }
        max.max(min.abs())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_absolute_sum() {
        assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
        assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    }
}
