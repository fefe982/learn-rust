// https://leetcode.com/problems/minimum-time-to-make-array-sum-at-most-x/
// 2809. Minimum Time to Make Array Sum At Most x
pub struct Solution;
impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let sum1 = nums1.iter().sum::<i32>();
        if sum1 <= x {
            return 0;
        }
        let sum2 = nums2.iter().sum::<i32>();
        let mut num = nums2.into_iter().zip(nums1).collect::<Vec<_>>();
        num.sort_unstable();
        let mut dp = vec![0; num.len() + 1];
        for t in 1..=num.len() {
            let mut ndp = vec![0; num.len() + 1];
            for i in t..=num.len() {
                ndp[i] = ndp[i - 1].max(dp[i - 1] + num[i - 1].0 * t as i32 + num[i - 1].1);
            }
            let val = sum1 + sum2 * t as i32 - ndp[num.len()];
            if val <= x {
                return t as i32;
            }
            dp = ndp;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_time() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
    }
}
