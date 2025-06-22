// https://leetcode.com/problems/decode-string/
// 394. Decode String
pub struct Solution;
impl Solution {
    fn decode_str(mut s: &[u8]) -> (String, &[u8]) {
        let mut r = String::new();
        let mut c = 0;
        while s.len() > 0 {
            match s[0] {
                b'0'..=b'9' => c = c * 10 + (s[0] - b'0') as usize,
                b'a'..=b'z' => r.push(s[0] as char),
                b'[' => {
                    let (sub, left) = Self::decode_str(&s[1..]);
                    r += &sub.repeat(c);
                    s = left;
                    c = 0;
                }
                b']' => return (r, s),
                _ => unreachable!(),
            }
            s = &s[1..];
        }
        (r, s)
    }
    pub fn decode_string(s: String) -> String {
        Self::decode_str(s.as_bytes()).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_string() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
    }
}
