// https://leetcode.com/problems/backspace-string-compare/
// 844. Backspace String Compare
pub struct Solution;
impl Solution {
    fn get_clean(s: String) -> Vec<char> {
        let mut chars = Vec::new();
        for c in s.chars() {
            if c == '#' {
                chars.pop();
            } else {
                chars.push(c);
            }
        }
        chars
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        Solution::get_clean(s) == Solution::get_clean(t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }
}
