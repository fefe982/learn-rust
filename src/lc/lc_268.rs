// https://leetcode.com/problems/missing-number/
// 268. Missing Number
pub struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.into_iter().enumerate().fold(0, |acc, (i, v)| acc ^ (i as i32) ^ v) ^ n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing_number() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
