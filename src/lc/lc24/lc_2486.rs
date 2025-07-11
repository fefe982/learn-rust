// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/
// 2486. Append Characters to String to Make Subsequence
pub struct Solution;
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut is = 0;
        let mut it = 0;
        while is < s.len() && it < t.len() {
            if s[is] == t[it] {
                it += 1;
            }
            is += 1;
        }
        (t.len() - it) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_append_characters() {
        assert_eq!(
            Solution::append_characters("coaching".to_string(), "coding".to_string()),
            4
        );
        assert_eq!(Solution::append_characters("abcde".to_string(), "a".to_string()), 0);
        assert_eq!(Solution::append_characters("z".to_string(), "abcde".to_string()), 5);
    }
}
