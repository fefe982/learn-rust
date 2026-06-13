// https://leetcode.com/problems/decode-the-message/
// 2325. Decode the Message
pub struct Solution;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut m = [0; 26];
        let mut c = b'a';
        for k in key.bytes() {
            if k != b' ' && m[(k - b'a') as usize] == 0 {
                m[(k - b'a') as usize] = c;
                c += 1;
            }
        }
        message
            .bytes()
            .map(|b| if b == b' ' { b' ' } else { m[(b - b'a') as usize] })
            .map(|b| b as char)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_message() {
        assert_eq!(
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret".to_string()
        );
        assert_eq!(
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly".to_string()
        );
    }
}
