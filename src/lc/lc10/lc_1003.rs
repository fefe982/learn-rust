// https://leetcode.com/problems/check-if-word-is-valid-after-substitutions/
// 1003. Check If Word Is Valid After Substitutions
pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            match c {
                b'a' => stack.push(c),
                b'b' => {
                    if let Some(t) = stack.last_mut() {
                        if *t != b'a' {
                            return false;
                        } else {
                            *t = b'b';
                        }
                    } else {
                        return false;
                    }
                }
                b'c' => {
                    if let Some(t) = stack.pop() {
                        if t != b'b' {
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
    fn is_valid() {
        assert_eq!(Solution::is_valid(String::from("aabcbc")), true);
        assert_eq!(Solution::is_valid(String::from("abcabcababcc")), true);
        assert_eq!(Solution::is_valid(String::from("abccbs")), false);
    }
}
