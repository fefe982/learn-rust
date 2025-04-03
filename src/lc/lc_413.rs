// https://leetcode.com/problems/arithmetic-slices/
// 413. Arithmetic Slices
pub struct Solution;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut count = 0;
        let mut cnt = 2;
        let mut diff = nums[1] - nums[0];
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == diff {
                cnt += 1;
            } else {
                count += (cnt - 2) * (cnt - 1) / 2;
                cnt = 2;
                diff = nums[i] - nums[i - 1];
            }
        }
        count + (cnt - 2) * (cnt - 1) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_arithmetic_slices() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
}
