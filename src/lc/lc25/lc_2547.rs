// https://leetcode.com/problems/minimum-cost-to-split-an-array/
// 2547. Minimum Cost to Split an Array
pub struct Solution;
impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut split = vec![vec![0; n]; n];
        for i in 0..n {
            let mut cnt = vec![0; n];
            let mut val = 0;
            for j in i..n {
                cnt[nums[j] as usize] += 1;
                if cnt[nums[j] as usize] == 2 {
                    val += 2;
                } else if cnt[nums[j] as usize] > 2 {
                    val += 1;
                }
                split[i][j] = val;
            }
        }
        let mut dp = vec![i32::MAX; n];
        for i in (0..n).rev() {
            dp[i] = split[i][n - 1] + k;
            for j in i..n - 1 {
                dp[i] = dp[i].min(dp[j + 1] + split[i][j] + k);
            }
        }
        dp[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 2), 6);
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 5), 10);
    }
}
