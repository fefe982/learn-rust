// https://leetcode.com/problems/decode-the-slanted-ciphertext/
// 2075. Decode the Slanted Ciphertext
pub struct Solution;
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let len = encoded_text.len();
        if len == 0 {
            return String::new();
        }
        let rows = rows as usize;
        let cols = len / rows;
        let b = encoded_text.as_bytes();
        let mut res = String::with_capacity(len);
        for c in 0..cols {
            for r in 0..rows {
                if c + r >= cols {
                    break;
                }
                res.push(b[r * cols + c + r] as char);
            }
        }
        while res.ends_with(' ') {
            res.pop();
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_ciphertext() {
        assert_eq!(
            Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
            "cipher".to_string()
        );
        assert_eq!(
            Solution::decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
            "i love leetcode".to_string()
        );
        assert_eq!(
            Solution::decode_ciphertext("coding".to_string(), 1),
            "coding".to_string()
        );
    }
}
