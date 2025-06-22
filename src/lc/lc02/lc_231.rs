// https://leetcode.com/problems/power-of-two/
// 231. Power of Two
pub struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_power_of_two() {
        assert_eq!(Solution::is_power_of_two(0), false);
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}
