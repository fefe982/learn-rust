// https://leetcode.com/problems/remove-element/
// 27. Remove Element
pub struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(nums: Vec<i32>, val: i32, expect: Vec<i32>) {
        let mut nums = nums;
        let len = Solution::remove_element(&mut nums, val);
        nums.truncate(len as usize);
        assert_sort_eq!(nums, expect);
    }
    #[test]
    fn test_remove_element() {
        check(vec![3, 2, 2, 3], 3, vec![2, 2]);
        check(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4]);
    }
}
