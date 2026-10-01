// https://leetcode.com/problems/sum-of-k-subarrays-with-length-at-least-m/
// 3473. Sum of K Subarrays With Length at Least M
pub struct Solution;
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        let neg_inf = i32::MIN / 2;
        let mut dp = vec![0; n + 1];
        let m = m as usize;
        let k = k as usize;
        for i in 1..=k {
            let mut ndp = vec![neg_inf; n + 1];
            let mut best = dp[0];
            for j in i * m..=n {
                ndp[j] = ndp[j].max(ndp[j - 1]);
                best = best.max(dp[j - m] - prefix[j - m]);
                ndp[j] = ndp[j].max(best + prefix[j]);
            }
            dp = ndp;
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum() {
        assert_eq!(Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2), 13);
        assert_eq!(Solution::max_sum(vec![-10, 3, -1, -2], 4, 1), -10);
    }
}
