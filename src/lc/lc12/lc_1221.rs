// https://leetcode.com/problems/split-a-string-in-balanced-strings/
// 1221. Split a String in Balanced Strings
pub struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut t = 0;
        let mut a = 0;
        for c in s.chars() {
            if c == 'R' {
                t += 1;
            } else {
                t -= 1;
            }
            if t == 0 {
                a += 1;
            }
        }
        a
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn balanced_string_split() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }
}
