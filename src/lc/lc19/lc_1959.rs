// https://leetcode.com/problems/minimum-total-space-wasted-with-k-resizing-operations/
// 1959. Minimum Total Space Wasted With K Resizing Operations
pub struct Solution;
impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        // waste[l][r]: wasted space if one fixed size serves nums[l..=r].
        let mut waste = vec![vec![0_i64; n]; n];
        for l in 0..n {
            let mut mx = 0_i64;
            let mut sum = 0_i64;
            for r in l..n {
                let v = nums[r] as i64;
                mx = mx.max(v);
                sum += v;
                waste[l][r] = mx * (r - l + 1) as i64 - sum;
            }
        }

        let inf = i64::MAX / 4;
        // dp[i][j]: min waste for first i elements with exactly j segments.
        let mut dp = vec![vec![inf; k + 2]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            let max_seg = (k + 1).min(i);
            for seg in 1..=max_seg {
                for p in (seg - 1)..i {
                    let prev = dp[p][seg - 1];
                    if prev == inf {
                        continue;
                    }
                    let cand = prev + waste[p][i - 1];
                    if cand < dp[i][seg] {
                        dp[i][seg] = cand;
                    }
                }
            }
        }

        dp[n][k + 1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_space_wasted_k_resizing() {
        assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20], 0), 10);
        assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1), 10);
        assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2), 15);
    }
}
