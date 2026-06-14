// https://leetcode.com/problems/minimum-sum-of-squared-difference/
// 2333. Minimum Sum of Squared Difference
pub struct Solution;
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diff = nums1
            .into_iter()
            .zip(nums2.into_iter())
            .map(|(a, b)| (a - b).abs() as i64)
            .collect::<Vec<_>>();
        diff.sort_unstable_by(|a, b| b.cmp(a));
        diff.push(0);
        let mut k = (k1 + k2) as i64;
        let mut i0 = 0;
        let mut i1 = 0;
        let mut n = diff[0];
        for i in 1..diff.len() {
            let d = diff[i - 1] - diff[i];
            let need = d * i as i64;
            if k >= need {
                i1 = i;
                n = diff[i];
                k -= need;
            } else {
                let dec = k / i as i64;
                n -= dec;
                i0 = k % i as i64;
                break;
            }
        }
        let mut sum = (n - 1) * (n - 1) * i0 + n * n * (i1 as i64 - i0 + 1);
        for i in i1 + 1..diff.len() {
            sum += diff[i] * diff[i];
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_sum_square_diff() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }
}
