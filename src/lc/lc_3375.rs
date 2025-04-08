// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/
// 3375. Minimum Operations to Make Array Values Equal to K
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        if k > nums[0] {
            return -1;
        }
        nums.dedup();
        if k == nums[0] {
            nums.len() as i32 - 1
        } else {
            nums.len() as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![5, 2, 5, 4, 5], 2), 2);
        assert_eq!(Solution::min_operations(vec![2, 1, 2], 2), -1);
    }
}
