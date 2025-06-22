// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// 26. Remove Duplicates from Sorted Array
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        let mut j = 1;
        while j < nums.len() {
            if nums[j] != nums[j - 1] {
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
    fn check(nums: Vec<i32>, expect: Vec<i32>) {
        let mut nums = nums;
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(nums[..len as usize], expect);
    }
    #[test]
    fn test_remove_duplicates() {
        check(vec![1, 1, 2], vec![1, 2]);
        check(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]);
    }
}
