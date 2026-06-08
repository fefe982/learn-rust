// https://leetcode.com/problems/maximum-total-subarray-value-i/
// 3689. Maximum Total Subarray Value I
pub struct Solution;
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for n in nums {
            max = max.max(n);
            min = min.min(n);
        }
        (max - min) as i64 * k as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_total_value() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}
