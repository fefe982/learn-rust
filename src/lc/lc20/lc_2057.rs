// https://leetcode.com/problems/smallest-index-with-equal-value/
// 2057. Smallest Index With Equal Value
pub struct Solution;
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, &v) in nums.iter().enumerate() {
            if i as i32 % 10 == v {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_equal() {
        assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
        assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
        assert_eq!(Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), -1);
    }
}
