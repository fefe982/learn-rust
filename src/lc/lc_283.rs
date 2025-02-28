// https://leetcode.com/problems/move-zeroes/
// 283. Move Zeroes
pub struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
        }
        for i in j..nums.len() {
            nums[i] = 0;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: Vec<i32>, expect: Vec<i32>) {
        let mut nums = nums;
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expect);
    }
    #[test]
    fn test_move_zeroes() {
        check(vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]);
        check(vec![0], vec![0]);
    }
}
