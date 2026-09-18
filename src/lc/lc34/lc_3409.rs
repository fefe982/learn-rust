// https://leetcode.com/problems/longest-subsequence-with-decreasing-adjacent-difference/
// 3409. Longest Subsequence With Limited Adjacent Difference
pub struct Solution;
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 300]; 301];
        for &num in &nums {
            let ns = num as usize;
            for n in 1..=300 {
                let d = (num - n).abs() as usize;
                dp[ns][d] = dp[ns][d].max(dp[n as usize][d] + 1);
            }
            for i in (0..299).rev() {
                dp[ns][i] = dp[ns][i].max(dp[ns][i + 1]);
            }
        }
        let mut ans = 0;
        for i in 1..=300 {
            ans = ans.max(dp[i][0]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subsequence() {
        assert_eq!(Solution::longest_subsequence(vec![2, 8, 8, 8, 1]), 4);
        assert_eq!(Solution::longest_subsequence(vec![16, 6, 3]), 3);
        assert_eq!(Solution::longest_subsequence(vec![6, 5, 3, 4, 2, 1]), 4);
        assert_eq!(Solution::longest_subsequence(vec![10, 20, 10, 19, 10, 20]), 5);
    }
}
