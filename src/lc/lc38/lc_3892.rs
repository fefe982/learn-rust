// https://leetcode.com/problems/minimum-operations-to-achieve-at-least-k-peaks/
// 3892. Minimum Operations to Achieve at Least K Peaks
pub struct Solution;
impl Solution {
    fn path_min_cost(weights: &[i64], pick: usize) -> i64 {
        const INF: i64 = i64::MAX / 4;
        let m = weights.len();
        if pick == 0 {
            return 0;
        }
        if pick > m.div_ceil(2) {
            return INF;
        }

        let mut dp0 = vec![INF; pick + 1];
        let mut dp1 = vec![INF; pick + 1];
        dp0[0] = 0;

        for &w in weights {
            let mut ndp0 = vec![INF; pick + 1];
            let mut ndp1 = vec![INF; pick + 1];
            for j in 0..=pick {
                ndp0[j] = dp0[j].min(dp1[j]);
                if j > 0 && dp0[j - 1] < INF {
                    ndp1[j] = dp0[j - 1] + w;
                }
            }
            dp0 = ndp0;
            dp1 = ndp1;
        }

        dp0[pick].min(dp1[pick])
    }

    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        const INF: i64 = i64::MAX / 4;

        let n = nums.len();
        let k = k as usize;
        if k == 0 {
            return 0;
        }

        let max_peaks = n / 2;
        if k > max_peaks {
            return -1;
        }

        if n == 2 {
            let w0 = (nums[1] + 1 - nums[0]).max(0) as i64;
            let w1 = (nums[0] + 1 - nums[1]).max(0) as i64;
            return if k == 1 { w0.min(w1) as i32 } else { -1 };
        }

        let mut weights = vec![0i64; n];
        for i in 0..n {
            let l = if i == 0 { n - 1 } else { i - 1 };
            let r = if i + 1 == n { 0 } else { i + 1 };
            let need = nums[l].max(nums[r]) + 1;
            weights[i] = (need - nums[i]).max(0) as i64;
        }

        // Case 1: do not take index 0, solve path [1..n-1].
        let case1 = Self::path_min_cost(&weights[1..], k);

        // Case 2: take index 0, then cannot take 1 and n-1, solve path [2..n-2] for k-1.
        let case2 = if k >= 1 {
            weights[0] + Self::path_min_cost(&weights[2..n - 1], k - 1)
        } else {
            INF
        };

        let ans = case1.min(case2);
        if ans >= INF {
            -1
        } else {
            ans as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 1, 2], 1), 1);
        assert_eq!(Solution::min_operations(vec![4, 5, 3, 6], 2), 0);
        assert_eq!(Solution::min_operations(vec![3, 7, 3], 2), -1);
        assert_eq!(Solution::min_operations(vec![1, 1, 1, 1], 2), 2);
        assert_eq!(Solution::min_operations(vec![1, 2], 1), 0);
        assert_eq!(Solution::min_operations(vec![2, 1], 1), 0);
        assert_eq!(Solution::min_operations(vec![2, 2], 1), 1);
        assert_eq!(Solution::min_operations(vec![5, 1, 5, 1, 5], 2), 1);
    }
}
