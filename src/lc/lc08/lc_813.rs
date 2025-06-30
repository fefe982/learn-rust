// https://leetcode.com/problems/largest-sum-of-averages/
// 813. Largest Sum of Averages
pub struct Solution;
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = vec![0; nums.len() + 1];
        let mut dp = vec![0.0; nums.len()];
        for (i, &num) in nums.iter().enumerate() {
            sum[i + 1] = sum[i] + num;
            dp[i] = sum[i + 1] as f64 / (i + 1) as f64;
        }
        let n = nums.len();
        for _ in 1..k {
            for i in (1..n).rev() {
                for j in 0..i {
                    dp[i] = dp[i].max(dp[j] + (sum[i + 1] - sum[j + 1]) as f64 / (i - j) as f64);
                }
            }
        }
        dp[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn largest_sum_of_averages() {
        assert_approx_eq!(Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3), 20.0, 1e-6);
        assert_approx_eq!(
            Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4),
            20.5,
            1e-6
        );
    }
}
