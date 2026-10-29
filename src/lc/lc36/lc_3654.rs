// https://leetcode.com/problems/minimum-sum-after-divisible-sum-deletions/
// 3654. Minimum Sum After Divisible Sum Deletions
pub struct Solution;
impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut dp = vec![i64::MAX; k as usize];
        dp[0] = 0;
        let mut sum = 0;
        for n in nums {
            sum += n as i64;
            let bucket = (sum % (k as i64)) as usize;
            sum = dp[bucket].min(sum);
            dp[bucket] = sum;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_array_sum() {
        assert_eq!(Solution::min_array_sum(vec![1, 1, 1], 2), 1);
        assert_eq!(Solution::min_array_sum(vec![3, 1, 4, 1, 5], 3), 5);
    }
}
