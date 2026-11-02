// https://leetcode.com/problems/minimum-operations-to-equalize-array/
// 3674. Minimum Operations to Equalize Array
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i] != nums[0] {
                return 1;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 2]), 1);
        assert_eq!(Solution::min_operations(vec![5, 5, 5]), 0);
    }
}
