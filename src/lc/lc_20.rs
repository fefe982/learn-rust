// https://leetcode.com/problems/valid-parentheses/
// 20. Valid Parentheses
pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for &b in s.as_bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                b')' => {
                    if let Some(l) = stack.pop() {
                        if l != b'(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                b']' => {
                    if let Some(l) = stack.pop() {
                        if l != b'[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                b'}' => {
                    if let Some(l) = stack.pop() {
                        if l != b'{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_k_lists() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
