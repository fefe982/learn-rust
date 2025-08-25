// https://leetcode.cn/problems/minimum-operations-to-exceed-threshold-value-i/
// 3065. Minimum Operations to Exceed Threshold Value I
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().filter(|&x| x < k).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 3);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 1), 0);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 9), 4);
    }
}
