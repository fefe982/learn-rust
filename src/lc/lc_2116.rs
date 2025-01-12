// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/
// 1918. Check if a Parentheses String Can Be Valid
pub struct Solution;
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut min = 0;
        let mut max = 0;
        for (c, l) in s.chars().zip(locked.chars()) {
            if l == '0' {
                if min > 0 {
                    min -= 1;
                } else if min == 0 {
                    min += 1;
                }
                max += 1;
            } else if c == '(' {
                min += 1;
                max += 1;
            } else {
                min -= 1;
                max -= 1;
            }
            if max < 0 {
                return false;
            }
            if min < 0 {
                min = max % 2;
            }
        }
        min == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_be_valid() {
        assert_eq!(Solution::can_be_valid("))()))".to_string(), "010100".to_string()), true);
        assert_eq!(Solution::can_be_valid("()()".to_string(), "0000".to_string()), true);
        assert_eq!(Solution::can_be_valid("(".to_string(), "0".to_string()), false);
    }
}
