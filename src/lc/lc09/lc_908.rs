// https://leetcode.com/problems/smallest-range-i/
// 908. Smallest Range I
pub struct Solution;
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (min, max) = nums
            .into_iter()
            .fold((i32::MAX, i32::MIN), |(min, max), x| (min.min(x), max.max(x)));
        (max - min - 2 * k).max(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_range_i() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
