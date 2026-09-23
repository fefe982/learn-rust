// https://leetcode.com/problems/smallest-index-with-digit-sum-equal-to-index/
// 3550. Smallest Index With Equal Digit Sum
pub struct Solution;
impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut sum = 0;
            let mut n = nums[i];
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            if sum == i as i32 {
                return sum;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_index() {
        assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
        assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
        assert_eq!(Solution::smallest_index(vec![1, 2, 3, 4]), -1);
    }
}
