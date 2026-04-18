// https://leetcode.com/problems/remove-one-element-to-make-the-array-strictly-increasing/
// 1909. Remove One Element to Make the Array Strictly Increasing
pub struct Solution;
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut cnt = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                cnt += 1;
                if cnt > 1 {
                    return false;
                }
                if i > 1 && nums[i] <= nums[i - 2] && i < nums.len() - 1 && nums[i + 1] <= nums[i - 1] {
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
    fn test_can_be_increasing() {
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
        assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
        assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
    }
}
