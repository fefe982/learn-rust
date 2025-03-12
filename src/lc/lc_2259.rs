// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
// 2259. Maximum Count of Positive Integers and Negative Integers
pub struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        nums.partition_point(|&x| x < 0)
            .max(nums.len() - nums.partition_point(|&x| x <= 0)) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_count() {
        assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
        assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
        assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4);
    }
}
