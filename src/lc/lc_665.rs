// https://leetcode.com/problems/non-decreasing-array/
// 665. Non-decreasing Array
pub struct Solution;
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut p = usize::MAX;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if p != usize::MAX {
                    return false;
                }
                p = i - 1;
                if i != 1 && nums[i - 2] > nums[i] && i != nums.len() - 1 && nums[i - 1] > nums[i + 1] {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_possibility() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
    }
}
