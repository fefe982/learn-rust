// https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
// 1309. Decrypt String from Alphabet to Integer Mapping
pub struct Solution;
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let s = s.as_bytes();
        let mut res = String::new();
        let mut i = 0;
        while i < s.len() {
            if (i + 2) < s.len() && s[i + 2] == b'#' {
                res.push(((s[i] - b'0') * 10 + s[i + 1] - b'0' - 1 + b'a') as char);
                i += 3;
            } else {
                res.push((s[i] - b'1' + b'a') as char);
                i += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn freq_alphabets() {
        assert_eq!(Solution::freq_alphabets("10#11#12".to_string()), "jkab".to_string());
        assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz".to_string());
    }
}
