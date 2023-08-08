// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
// 1749. Maximum Absolute Sum of Any Subarray
pub struct Solution;
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        let mut min = sum.clone();
        let mut max = sum.clone();
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
            min[i + 1] = min[i].min(sum[i + 1]);
            max[i + 1] = max[i].max(sum[i + 1]);
        }
        let mut mx = sum[nums.len()];
        let mut mn = sum[nums.len()];
        let mut mxdiff = 0;
        let mut mndiff = 0;
        for i in (0..nums.len()).rev() {
            mxdiff = mxdiff.max(mx - min[i]);
            mndiff = mndiff.min(mn - max[i]);
            mx = mx.max(sum[i]);
            mn = mn.min(sum[i]);
        }
        mxdiff.abs().max(mndiff.abs())
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
