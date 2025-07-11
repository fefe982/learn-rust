// https://leetcode.com/problems/apply-operations-to-an-array/
// 2460. Apply Operations to an Array
pub struct Solution;
impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut idx = 0;
        for i in 1..nums.len() {
            if nums[i - 1] != 0 {
                if nums[i] == nums[i - 1] {
                    nums[i - 1] *= 2;
                    nums[i] = 0;
                }
                if idx != i - 1 {
                    nums[idx] = nums[i - 1];
                }
                idx += 1;
            }
        }
        if nums[nums.len() - 1] != 0 {
            if idx != nums.len() - 1 {
                nums[idx] = nums[nums.len() - 1];
            }
            idx += 1;
        }
        for i in idx..nums.len() {
            nums[i] = 0;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apply_operations() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }
}
