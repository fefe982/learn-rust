// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
// 153. Find Minimum in Rotated Sorted Array
pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 || nums[0] < nums[n - 1] {
            return nums[0];
        }
        let mut l = 0;
        let mut r = n - 1;
        while l + 1 < r {
            let m = (l + r) / 2;
            if nums[m] > nums[r] {
                l = m;
            } else {
                r = m;
            }
        }
        nums[r]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
