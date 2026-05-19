// https://leetcode.com/problems/find-all-lonely-numbers-in-the-array/
// 2150. Find All Lonely Numbers in the Array
pub struct Solution;
impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if (i == 0 || nums[i - 1] + 1 < nums[i]) && (i == nums.len() - 1 || nums[i] + 1 < nums[i + 1]) {
                res.push(nums[i]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_lonely() {
        assert_sort_eq!(Solution::find_lonely(vec![10, 6, 5, 8]), vec![8, 10]);
        assert_sort_eq!(Solution::find_lonely(vec![1, 3, 5, 3]), vec![1, 5]);
    }
}
