// https://leetcode.com/problems/majority-element/
// 169. Majority Element
pub struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
