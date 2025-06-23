// https://leetcode.com/problems/find-the-difference-of-two-arrays/
// 2215. Find the Difference of Two Arrays
pub struct Solution;
impl Solution {
    fn inc(nums: &Vec<i32>, mut i: usize) -> usize {
        let n = nums[i];
        i += 1;
        while i < nums.len() && nums[i] == n {
            i += 1;
        }
        i
    }
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort_unstable();
        nums1.push(i32::MAX);
        nums2.sort_unstable();
        nums2.push(i32::MAX);
        let mut i0 = 0;
        let mut i1 = 0;
        let mut v: Vec<Vec<i32>> = vec![Vec::new(), Vec::new()];
        while i0 < nums1.len() && i1 < nums2.len() {
            match nums1[i0].cmp(&nums2[i1]) {
                std::cmp::Ordering::Less => {
                    v[0].push(nums1[i0]);
                    i0 = Self::inc(&nums1, i0);
                }
                std::cmp::Ordering::Equal => {
                    i0 = Self::inc(&nums1, i0);
                    i1 = Self::inc(&nums2, i1);
                }
                std::cmp::Ordering::Greater => {
                    v[1].push(nums2[i1]);
                    i1 = Self::inc(&nums2, i1);
                }
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_difference() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            vec_vec![[1, 3], [4, 6]]
        );
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec_vec![[3], []]
        );
    }
}
