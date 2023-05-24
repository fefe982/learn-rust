// https://leetcode.com/problems/maximum-subsequence-score/
// 2542. Maximum Subsequence Score
pub struct Solution;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut heap1 = std::collections::BinaryHeap::new();
        let mut heap2 = std::collections::BinaryHeap::new();
        let mut used_min = vec![false; nums1.len()];
        let mut in_sum = vec![false; nums1.len()];
        for (i, (&n1, &n2)) in nums1.iter().zip(nums2.iter()).enumerate() {
            heap1.push((n1, i));
            heap2.push((-n2, i));
        }
        let mut s0 = 0;
        let mut max = i64::MIN;
        for _ in 1..k {
            let (n, i) = heap1.pop().unwrap();
            in_sum[i] = true;
            s0 += n as i64;
        }
        for _ in 0..(nums1.len() as i32 - k + 1) {
            let (n, i) = heap2.pop().unwrap();
            used_min[i] = true;
            if in_sum[i] {
                s0 -= nums1[i] as i64;
                while let Some((nn, ii)) = heap1.pop() {
                    if used_min[ii] {
                        continue;
                    }
                    in_sum[ii] = true;
                    s0 += nn as i64;
                    break;
                }
            }
            max = std::cmp::max(max, -n as i64 * (s0 + nums1[i] as i64));
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3),
            12
        );
        assert_eq!(
            Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1),
            30
        );
    }
}
