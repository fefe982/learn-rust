// https://leetcode.com/problems/form-smallest-number-from-two-digit-arrays/
// 2605. Form Smallest Number From Two Digit Arrays
pub struct Solution;
impl Solution {
    pub fn min_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        if nums1[0] < nums2[0] {
            nums1[0] * 10 + nums2[0]
        } else {
            nums2[0] * 10 + nums1[0]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_number() {
        assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
        assert_eq!(Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
    }
}
