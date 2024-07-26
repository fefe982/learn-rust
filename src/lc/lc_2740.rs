// https://leetcode.com/problems/find-the-value-of-the-partition/
// 2740. Find the Value of the Partition
pub struct Solution;
impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut min = i32::MAX;
        for i in 1..nums.len() {
            min = min.min(nums[i] - nums[i - 1]);
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_value_of_partition() {
        assert_eq!(Solution::find_value_of_partition(vec![1, 3, 2, 4]), 1);
        assert_eq!(Solution::find_value_of_partition(vec![100, 1, 10]), 9);
    }
}
