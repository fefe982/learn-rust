// https://leetcode.com/problems/minimum-operations-to-transform-array/
// 3724. Minimum Operations to Transform Array
pub struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut add = i32::MAX;
        let t = *nums2.last().unwrap();
        let mut step = 1;
        for i in 0..nums1.len() {
            step += (nums1[i] - nums2[i]).abs() as i64;
            if (nums1[i] <= t && nums2[i] >= t) || (nums1[i] >= t && nums2[i] <= t) {
                add = 0;
            } else {
                add = add.min((nums1[i] - t).abs()).min((nums2[i] - t).abs());
            }
        }
        step + add as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 8], vec![1, 7, 3]), 4);
        assert_eq!(Solution::min_operations(vec![1, 3, 6], vec![2, 4, 5, 3]), 4);
        assert_eq!(Solution::min_operations(vec![2], vec![3, 4]), 3);
    }
}
