// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/
// 3381. Maximum Subarray Sum with Length Divisible by K
pub struct Solution;
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut pre = vec![i64::MAX / 2; k];
        pre[k - 1] = 0;
        let mut ans = i64::MIN;
        let mut sum = 0i64;
        for (i, x) in nums.into_iter().enumerate() {
            let idx = i % k;
            let x = x as i64;
            sum += x;
            ans = ans.max(sum - pre[idx]);
            pre[idx] = pre[idx].min(sum);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_subarray_sum() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
        assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
        assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
    }
}
