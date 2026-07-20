// https://leetcode.com/problems/longest-non-decreasing-subarray-from-two-arrays/
// 2771. Longest Non-decreasing Subarray from Two Arrays
pub struct Solution;
impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max = 1;
        let mut l1 = 1;
        let mut l2 = 1;
        for i in (0..nums1.len() - 1).rev() {
            let mut nl1 = 1;
            if nums1[i] <= nums1[i + 1] {
                nl1 = l1 + 1;
            }
            if nums1[i] <= nums2[i + 1] {
                nl1 = nl1.max(l2 + 1);
            }
            let mut nl2 = 1;
            if nums2[i] <= nums1[i + 1] {
                nl2 = l1 + 1;
            }
            if nums2[i] <= nums2[i + 1] {
                nl2 = nl2.max(l2 + 1);
            }
            max = max.max(nl1.max(nl2));
            l1 = nl1;
            l2 = nl2;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_non_decreasing_length() {
        Solution::max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]);
        Solution::max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4]);
        Solution::max_non_decreasing_length(vec![1, 1], vec![2, 2]);
    }
}
