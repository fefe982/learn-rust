// https://leetcode.com/problems/sum-of-subarray-ranges/
// 2104. Sum of Subarray Ranges
pub struct Solution;
impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut min = nums[i];
            let mut max = nums[i];
            for j in i..nums.len() {
                min = min.min(nums[j]);
                max = max.max(nums[j]);
                res += (max - min) as i64;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub_array_ranges() {
        assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
    }
}
