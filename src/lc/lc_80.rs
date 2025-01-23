// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
// 80. Remove Duplicates from Sorted Array II
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: &mut Vec<i32>, ans: Vec<i32>) {
        assert_eq!(Solution::remove_duplicates(nums), ans.len() as i32);
        assert_eq!(nums[..ans.len()], ans);
    }
    #[test]
    fn test_remove_duplicates() {
        check(&mut vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3]);
        check(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3], vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
