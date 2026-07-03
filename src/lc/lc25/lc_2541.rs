// https://leetcode.com/problems/minimum-operations-to-make-array-equal-ii/
// 2541. Minimum Operations to Make Array Equal II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            return if nums1 == nums2 { 0 } else { -1 };
        }
        let mut diff = 0;
        let mut ans = 0;
        let k = k as i64;
        for (&a, &b) in nums1.iter().zip(nums2.iter()) {
            let d = a as i64 - b as i64;
            if d % k != 0 {
                return -1;
            }
            diff += d / k;
            ans += d.abs() / k;
        }
        if diff != 0 {
            return -1;
        }
        ans / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 2), 3);
        assert_eq!(Solution::min_operations(vec![4, 3, 1, 4], vec![3, 8, 5, 2], 1), -1);
    }
}
