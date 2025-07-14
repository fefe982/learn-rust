// https://leetcode.com/problems/valid-word/description/
// 3136. Valid Word
pub struct Solution;
impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut v = 0;
        let mut c = 0;
        let mut l = 0;
        for ch in word.chars() {
            if ch == 'a'
                || ch == 'e'
                || ch == 'i'
                || ch == 'o'
                || ch == 'u'
                || ch == 'A'
                || ch == 'E'
                || ch == 'I'
                || ch == 'O'
                || ch == 'U'
            {
                v += 1;
            } else if ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') {
                c += 1;
            } else if !('0' <= ch && ch <= '9') {
                return false;
            }
            l += 1;
        }
        l >= 3 && v >= 1 && c >= 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("234Adas".to_string()), true);
        assert_eq!(Solution::is_valid("b3".to_string()), false);
        assert_eq!(Solution::is_valid("a3$e".to_string()), false);
    }
}
