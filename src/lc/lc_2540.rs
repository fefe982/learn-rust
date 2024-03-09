// https://leetcode.com/problems/minimum-common-value/
// 2540. Minimum Common Value
pub struct Solution;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            }
            if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_common() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
