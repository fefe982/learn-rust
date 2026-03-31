// https://leetcode.com/problems/minimum-absolute-sum-difference/
// 1818. Minimum Absolute Sum Difference
pub struct Solution;
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut sorted = nums1.clone();
        sorted.sort_unstable();

        let mut total: i64 = 0;
        let mut best_gain: i64 = 0;

        for (&a, &b) in nums1.iter().zip(nums2.iter()) {
            let cur = (a - b).abs() as i64;
            total += cur;

            let idx = sorted.partition_point(|&x| x < b);

            if idx < sorted.len() {
                let cand = (sorted[idx] - b).abs() as i64;
                best_gain = best_gain.max(cur - cand);
            }
            if idx > 0 {
                let cand = (sorted[idx - 1] - b).abs() as i64;
                best_gain = best_gain.max(cur - cand);
            }
        }

        ((total - best_gain) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_absolute_sum_diff() {
        assert_eq!(Solution::min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]), 3);
        assert_eq!(
            Solution::min_absolute_sum_diff(vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10]),
            0
        );
        assert_eq!(
            Solution::min_absolute_sum_diff(vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4]),
            20
        );
    }
}
