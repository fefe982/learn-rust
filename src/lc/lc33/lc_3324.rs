// https://leetcode.com/problems/find-the-sequence-of-strings-appeared-on-the-screen/
// 3324. Find the Sequence of Strings Appeared on the Screen
pub struct Solution;
impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        while s != target {
            if target.starts_with(&s) {
                s.push('a');
                ans.push(s.clone());
            } else {
                let c = s.pop().unwrap();
                s.push((c as u8 + 1) as char);
                ans.push(s.clone());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn string_sequence() {
        assert_eq!(
            Solution::string_sequence(String::from("abc")),
            vec_str!["a", "aa", "ab", "aba", "abb", "abc"]
        );
        assert_eq!(
            Solution::string_sequence(String::from("he")),
            vec_str!["a", "b", "c", "d", "e", "f", "g", "h", "ha", "hb", "hc", "hd", "he"]
        );
    }
}
