// https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// 405. Convert a Number to Hexadecimal
pub struct Solution;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut n = num as u32;
        if n == 0 {
            return "0".to_string();
        }
        let mut hex = String::new();
        while n != 0 {
            let digit = n % 16;
            hex.push(match digit {
                0..=9 => (b'0' + digit as u8) as char,
                _ => (b'a' + digit as u8 - 10) as char,
            });
            n /= 16;
        }
        hex.chars().rev().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_hex() {
        assert_eq!(Solution::to_hex(26), "1a".to_string());
        assert_eq!(Solution::to_hex(-1), "ffffffff".to_string());
    }
}
