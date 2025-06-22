// https://leetcode.com/problems/merge-sorted-array/
// 88. Merge Sorted Array
pub struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut s = m as usize + n as usize - 1;
        let mut l = m - 1;
        let mut r = n - 1;
        while l >= 0 && r >= 0 {
            if nums1[l as usize] > nums2[r as usize] {
                nums1[s] = nums1[l as usize];
                s -= 1;
                l -= 1;
            } else {
                nums1[s] = nums2[r as usize];
                s -= 1;
                r -= 1;
            }
        }
        while r >= 0 {
            nums1[s] = nums2[r as usize];
            if s == 0 {
                break;
            }
            s -= 1;
            r -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge() {
        let mut v = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut v, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(v, vec![1, 2, 2, 3, 5, 6]);
        v = vec![1];
        Solution::merge(&mut v, 1, &mut vec![], 0);
        assert_eq!(v, vec![1]);
        v = vec![0];
        Solution::merge(&mut v, 0, &mut vec![1], 1);
        assert_eq!(v, vec![1]);
    }
}
