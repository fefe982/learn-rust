// https://leetcode.com/problems/reverse-prefix-of-word/
// 2000. Reverse Prefix of Word
pub struct Solution;
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut idx = usize::MAX;
        let mut v = vec![];
        for c in word.chars() {
            if c == ch && idx == usize::MAX {
                idx = v.len();
            }
            v.push(c);
        }
        if idx == usize::MAX {
            return word;
        }
        return v[0..=idx].iter().rev().chain(v[idx + 1..].iter()).collect();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_prefix() {
        assert_eq!(Solution::reverse_prefix("abcdefd".to_string(), 'd'), "dcbaefd");
        assert_eq!(Solution::reverse_prefix("xyxzxe".to_string(), 'z'), "zxyxxe");
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }
}
