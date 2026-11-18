// https://leetcode.com/problems/longest-non-decreasing-subarray-after-replacing-at-most-one-element/
// 3738. Longest Non-Decreasing Subarray After Replacing at Most One Element
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];
        let mut max = 0;
        left[0] = 1;
        right[nums.len() - 1] = 1;
        for i in 1..nums.len() {
            if nums[i] >= nums[i - 1] {
                left[i] = left[i - 1] + 1;
                max = max.max(left[i]);
            } else {
                left[i] = 1;
            }
            if nums[nums.len() - i] >= nums[nums.len() - i - 1] {
                right[nums.len() - i - 1] = right[nums.len() - i] + 1;
            } else {
                right[nums.len() - i - 1] = 1;
            }
        }
        for i in 1..nums.len() - 1 {
            if nums[i + 1] >= nums[i - 1] {
                max = max.max(left[i - 1] + right[i + 1] + 1);
            } else {
                max = max.max(left[i - 1] + 1).max(right[i + 1] + 1);
            }
        }
        if nums.len() > 1 {
            max = max.max(left[nums.len() - 2] + 1).max(right[1] + 1);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![7, 1, -8]), 2);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 1, 2]), 4);
        assert_eq!(Solution::longest_subarray(vec![2, 2, 2, 2, 2]), 5);
    }
}
