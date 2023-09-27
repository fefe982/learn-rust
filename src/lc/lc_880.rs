// https://leetcode.com/problems/decoded-string-at-index/
// 880. Decoded String at Index
pub struct Solution;
impl Solution {
    pub fn decode_str(s: &[u8], k: usize) -> String {
        let mut i = 0;
        for &c in s {
            if c >= b'a' && c <= b'z' {
                if i == k {
                    return String::from(c as char);
                }
                i += 1;
            } else if c >= b'2' && c <= b'9' {
                let cnt = (c - b'0') as usize;
                let mul = i * cnt;
                if mul <= k {
                    i = mul;
                } else {
                    return Self::decode_str(s, k % i);
                }
            }
        }
        "".to_string()
    }
    pub fn decode_at_index(s: String, k: i32) -> String {
        Self::decode_str(s.as_bytes(), k as usize - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_at_index() {
        assert_eq!(Solution::decode_at_index("leet2code3".to_string(), 10), "o");
        assert_eq!(Solution::decode_at_index("ha22".to_string(), 5), "h");
        assert_eq!(
            Solution::decode_at_index("a2345678999999999999999".to_string(), 1),
            "a"
        );
    }
}
