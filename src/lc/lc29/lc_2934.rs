// https://leetcode.com/problems/minimum-operations-to-maximize-last-elements-in-arrays/
// 2934. Minimum Operations to Maximize the Last Element in an Array
pub struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let e1 = nums1[nums1.len() - 1];
        let e2 = nums2[nums2.len() - 1];
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 0..nums1.len() {
            let b1 = nums1[i] <= e1 && nums2[i] <= e2;
            let b2 = nums1[i] <= e2 && nums2[i] <= e1;
            if !b1 && !b2 {
                return -1;
            }
            if !b1 && b2 {
                c1 += 1;
            }
            if b1 && !b2 {
                c2 += 1;
            }
        }
        c1.min(c2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 2, 7], vec![4, 5, 3]), 1);
        assert_eq!(Solution::min_operations(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]), 2);
        assert_eq!(Solution::min_operations(vec![1, 5, 4], vec![2, 5, 3]), -1);
    }
}
