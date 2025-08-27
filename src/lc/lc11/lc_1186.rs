// https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/
// 1186. Maximum Subarray Sum with One Deletion
pub struct Solution;
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut dp = (arr[0], i32::MIN);
        let mut max = arr[0];
        for i in 1..arr.len() {
            dp = (dp.0.max(0) + arr[i], dp.1.max(dp.0 - arr[i - 1]) + arr[i]);
            max = max.max(dp.0).max(dp.1);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_sum() {
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
        assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
        assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
    }
}
