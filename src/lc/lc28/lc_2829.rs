// https://leetcode.com/problems/determine-the-minimum-sum-of-a-k-avoiding-array/
// 2829. Determine the Minimum Sum of a k-avoiding Array
pub struct Solution;
impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        if n <= k / 2 {
            n * (n + 1) / 2
        } else {
            (k / 2) * (k / 2 + 1) / 2 + (k * 2 + n - k / 2 - 1) * (n - k / 2) / 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(5, 4), 18);
        assert_eq!(Solution::minimum_sum(2, 6), 3);
    }
}
