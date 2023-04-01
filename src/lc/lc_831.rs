// https://leetcode.com/problems/masking-personal-information/
// 831. Masking Personal Information
pub struct Solution;
impl Solution {
    fn to_lower(c: u8) -> u8 {
        if c >= b'A' && c <= b'Z' {
            c - b'A' + b'a'
        } else {
            c
        }
    }
    pub fn mask_pii(s: String) -> String {
        let s = s.as_bytes();
        if (s[0] >= b'a' && s[0] <= b'z') || (s[0] >= b'A' && s[0] <= b'Z') {
            let mut r = vec![Self::to_lower(s[0]), b'*', b'*', b'*', b'*', b'*'];
            let mut b = s[0];
            let mut domain = false;
            for idx in 1..s.len() {
                if domain == false {
                    if s[idx] == b'@' {
                        domain = true;
                        r.push(Self::to_lower(b));
                        r.push(b'@');
                    } else {
                        b = s[idx];
                    }
                } else {
                    r.push(Self::to_lower(s[idx]))
                }
            }
            String::from_utf8(r).unwrap()
        } else {
            let mut r = Vec::new();
            for idx in 0..s.len() {
                if s[idx] >= b'0' && s[idx] <= b'9' {
                    r.push(s[idx]);
                }
            }
            let mut prefix = "";
            if r.len() == 11 {
                prefix = "+*-";
            } else if r.len() == 12 {
                prefix = "+**-";
            } else if r.len() == 13 {
                prefix = "+***-"
            }
            String::from_utf8(
                prefix
                    .as_bytes()
                    .iter()
                    .chain(b"***-***-".iter())
                    .chain(r[r.len() - 4..].iter())
                    .cloned()
                    .collect(),
            )
            .unwrap()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mask_pii() {
        assert_eq!(
            Solution::mask_pii(String::from("LeetCode@LeetCode.com")),
            String::from("l*****e@leetcode.com")
        );
        assert_eq!(
            Solution::mask_pii(String::from("AB@qq.com")),
            String::from("a*****b@qq.com")
        );
        assert_eq!(
            Solution::mask_pii(String::from("1(234)567-890")),
            String::from("***-***-7890")
        );
    }
}
