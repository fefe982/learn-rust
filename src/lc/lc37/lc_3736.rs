// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-iii/
// 3736. Minimum Moves to Equal Array Elements III
pub struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = nums[0];
        for i in 1..nums.len() {
            sum += nums[0] - nums[i];
            max = max.max(nums[i]);
        }
        sum + (max - nums[0]) * (nums.len() as i32)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec![2, 1, 3]), 3);
        assert_eq!(Solution::min_moves(vec![4, 4, 5]), 2);
    }
}
