// https://leetcode.com/problems/valid-parenthesis-string/
// 678. Valid Parenthesis String
pub struct Solution;
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min = 0;
        let mut max = 0;
        for c in s.chars() {
            if c == '(' {
                min += 1;
                max += 1;
            } else if c == ')' {
                max -= 1;
                if max < 0 {
                    return false;
                }
                min = 0.max(min - 1);
            } else {
                min = 0.max(min - 1);
                max += 1;
            }
        }
        min == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_valid_string() {
        assert_eq!(Solution::check_valid_string(String::from("()")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*)")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*))")), true);
    }
}
