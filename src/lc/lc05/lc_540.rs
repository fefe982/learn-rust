// https://leetcode.cn/problems/single-element-in-a-sorted-array/
// 540. Single Element in a Sorted Array
pub struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        for i in 0..len / 2 {
            if 2 * i + 1 >= nums.len() {
                return nums[i * 2];
            }
            if nums[2 * i] == nums[2 * i + 1] {
                continue;
            } else {
                return nums[2 * i];
            }
        }
        nums[len - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_non_duplicate() {
        assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
    }
}
