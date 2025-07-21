// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/
// 1608. Special Array With X Elements Greater Than or Equal X
pub struct Solution;
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        for i in 0..=nums.len() {
            if i > 0 && nums[i - 1] < i as i32 {
                continue;
            }
            if i < nums.len() && nums[i] >= i as i32 {
                continue;
            }
            return i as i32;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_special_array() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
