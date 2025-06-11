// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/
// 3423. Maximum Difference Between Adjacent Elements in a Circular Array
pub struct Solution;
impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            max = max.max((nums[(i + 1) % nums.len()] - nums[i]).abs());
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_adjacent_distance() {
        assert_eq!(Solution::max_adjacent_distance(vec![1, 2, 4]), 3);
        assert_eq!(Solution::max_adjacent_distance(vec![-5, -10, -5]), 5);
    }
}
