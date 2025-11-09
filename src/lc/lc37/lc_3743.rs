// https://leetcode.com/problems/maximize-cyclic-partition-score/
// 3743. Maximize Cyclic Partition Score
pub struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let mut imax = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[imax] {
                imax = i;
            }
        }
        let len = nums.len();
        let k = k as usize;
        let mut mx = i64::MIN;
        for j in 0..2 {
            let mut dp = vec![vec![i32::MIN as i64; 3]; k + 1];
            dp[0][0] = 0;
            for i in 0..nums.len() {
                let num = nums[(i + imax + j) % len] as i64;
                for l in (1..=k).rev() {
                    dp[l][1] = dp[l][1].max(dp[l - 1][0] + num);
                    dp[l][2] = dp[l][2].max(dp[l - 1][0] - num);
                    dp[l][0] = dp[l][0].max(dp[l][1] - num).max(dp[l][2] + num);
                }
            }
            mx = dp.into_iter().skip(1).fold(mx, |a, b| a.max(b[0]));
        }
        mx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_score() {
        assert_eq!(Solution::maximum_score(vec![1], 1), 0);
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 2), 3);
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 1), 2);
        assert_eq!(Solution::maximum_score(vec![1, 2, 3, 3], 4), 3);
    }
}
