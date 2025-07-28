// https://leetcode.com/problems/long-pressed-name/
// 925. Long Pressed Name
pub struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name = name.as_bytes();
        let typed = typed.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < name.len() && j < typed.len() {
            if name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed[j] == typed[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        if i < name.len() {
            return false;
        }
        while j < typed.len() {
            if j > 0 && typed[j] == typed[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_long_pressed_name() {
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
            false
        );
    }
}
