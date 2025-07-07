// https://leetcode.com/problems/make-the-xor-of-all-segments-equal-to-zero/
// 1787. Make the XOR of All Segments Equal to Zero
pub struct Solution;
impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut freq = vec![vec![0; 1024]; k];
        let mut dp = vec![nums.len() as i32; 1024];
        dp[0] = 0;
        for (i, &n) in nums.iter().enumerate() {
            freq[i % k][n as usize] += 1;
        }
        let mut min_chg = 0;
        for i in 0..k {
            let mut ndp = vec![i32::MAX; 1024];
            let mut n_min_chg = i32::MAX;
            let ni = (nums.len() - 1 - i) / k + 1;
            for j in 0..ni {
                for l in 0..1024 {
                    ndp[l] =
                        ndp[l].min(dp[l ^ nums[i + j * k] as usize] + ni as i32 - freq[i][nums[i + j * k] as usize]);
                }
            }
            for l in 0..1024 {
                ndp[l] = ndp[l].min(min_chg + ni as i32);
                n_min_chg = n_min_chg.min(ndp[l]);
            }
            dp = ndp;
            min_chg = n_min_chg;
        }
        dp[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_changes() {
        assert_eq!(Solution::min_changes(vec![1, 2, 0, 3, 0], 1), 3);
        assert_eq!(Solution::min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
        assert_eq!(Solution::min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    }
}
