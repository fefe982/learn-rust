// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// 1312. Minimum Insertion Steps to Make a String Palindrome
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    fn min_ins<'a>(s: &'a [u8], cache: &mut HashMap<&'a [u8], i32>) -> i32 {
        let len = s.len();
        if len == 1 || len == 0 {
            0
        } else if let Some(cnt) = cache.get(s) {
            *cnt
        } else {
            let cnt = if s[0] == s[len - 1] {
                Self::min_ins(&s[1..len - 1], cache)
            } else {
                std::cmp::min(
                    Self::min_ins(&s[..len - 1], cache),
                    Self::min_ins(&s[1..], cache),
                ) + 1
            };
            cache.insert(s, cnt);
            cnt
        }
    }
    pub fn min_insertions(s: String) -> i32 {
        Self::min_ins(s.as_bytes(), &mut HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_insertions() {
        assert_eq!(Solution::min_insertions(String::from("zzazz")), 0);
        assert_eq!(Solution::min_insertions(String::from("mbadm")), 2);
        assert_eq!(Solution::min_insertions(String::from("leetcode")), 5);
    }
}
