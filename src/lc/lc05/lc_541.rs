// https://leetcode.com/problems/reverse-string-ii/
// 541. Reverse String II
pub struct Solution;
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let k = k as usize;
        let mut i = 0;
        while i < s.len() {
            let j = (i + k).min(s.len()) - 1;
            s[i..=j].reverse();
            i += 2 * k;
        }
        String::from_utf8(s).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_str() {
        assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
        assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd".to_string());
    }
}
