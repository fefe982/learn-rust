// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
// 1910. Remove All Occurrences of a Substring
pub struct Solution;
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s.as_bytes().to_vec();
        let part = part.as_bytes();
        let mut lps = vec![0; part.len()];
        let mut len = 0;
        let mut i = 1;
        while i < part.len() {
            if part[i] == part[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        let mut v = vec![0; s.len() + 1];
        let mut i = 0;
        let mut j = 0;
        while i < s.len() {
            if s[i] == part[j] {
                j += 1;
                if j == part.len() {
                    i = i + 1 - part.len();
                    s.drain(i..i + part.len());
                    j = v[i];
                } else {
                    i += 1;
                    v[i] = j;
                }
            } else {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                    v[i] = j;
                }
            }
        }
        String::from_utf8(s).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_occurrences() {
        assert_eq!(Solution::remove_occurrences("a".to_string(), "aa".to_string()), "a");
        assert_eq!(
            Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
            "dab"
        );
        assert_eq!(
            Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
            "ab"
        );
        assert_eq!(Solution::remove_occurrences("abab".to_string(), "ab".to_string()), "");
    }
}
