// https://leetcode.com/problems/clear-digits/
// 3174. Clear Digits
pub struct Solution;
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut ret = String::new();
        for c in s.chars() {
            if c.is_digit(10) {
                ret.pop();
            } else {
                ret.push(c);
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clear_digits() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(Solution::clear_digits("cb34".to_string()), "cb34".to_string());
    }
}
