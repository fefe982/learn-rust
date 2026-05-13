// https://leetcode.com/problems/a-number-after-a-double-reversal/
// 2119. A Number After a Double Reversal
pub struct Solution;
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_same_after_reversals() {
        assert_eq!(Solution::is_same_after_reversals(526), true);
        assert_eq!(Solution::is_same_after_reversals(1800), false);
        assert_eq!(Solution::is_same_after_reversals(0), true);
    }
}
