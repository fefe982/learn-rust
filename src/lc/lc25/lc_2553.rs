// https://leetcode.com/problems/separate-the-digits-in-an-array/
// 2553. Separate the Digits in an Array
pub struct Solution;
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for n in nums {
            let mut stack = Vec::new();
            let mut x = n;
            while x > 0 {
                stack.push(x % 10);
                x /= 10;
            }
            while let Some(d) = stack.pop() {
                ans.push(d);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_separate_digits() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
        assert_eq!(Solution::separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    }
}
