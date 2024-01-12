// https://leetcode.com/problems/determine-if-string-halves-are-alike/
// 1704. Determine if String Halves Are Alike
pub struct Solution;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = 0;
        for i in 0..n / 2 {
            let c = s[i];
            if c == b'a'
                || c == b'e'
                || c == b'i'
                || c == b'o'
                || c == b'u'
                || c == b'A'
                || c == b'E'
                || c == b'I'
                || c == b'O'
                || c == b'U'
            {
                cnt += 1;
            }
        }
        for i in n / 2..n {
            let c = s[i];
            if c == b'a'
                || c == b'e'
                || c == b'i'
                || c == b'o'
                || c == b'u'
                || c == b'A'
                || c == b'E'
                || c == b'I'
                || c == b'O'
                || c == b'U'
            {
                cnt -= 1;
            }
            if cnt < 0 {
                return false;
            }
        }
        cnt == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_halves_are_alike() {
        assert_eq!(Solution::halves_are_alike("book".to_string()), true);
        assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
    }
}
