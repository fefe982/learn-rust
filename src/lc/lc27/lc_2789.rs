// https://leetcode.com/problems/largest-element-in-an-array-after-merge-operations/
// 2789. Largest Element in an Array after Merge Operations
pub struct Solution;
impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .rev()
            .fold((0, 0), |(max, last), n| {
                let n = n as i64;
                let last = if n <= last { last + n } else { n };
                (max.max(last), last)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_array_value() {
        assert_eq!(Solution::max_array_value(vec![2, 3, 7, 9, 3]), 21);
        assert_eq!(Solution::max_array_value(vec![5, 3, 3]), 11);
    }
}
