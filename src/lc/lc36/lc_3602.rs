// https://leetcode.com/problems/hexadecimal-and-hexatrigesimal-conversion/
// 3602. Hexadecimal and Hexatrigesimal Conversion
pub struct Solution;
impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let n2 = n * n;
        let mut n3 = n2 * n;
        let n2s = format!("{:X}", n2);
        let mut n3s = vec![];
        while n3 > 0 {
            let d = n3 % 36;
            if d < 10 {
                n3s.push((d as u8 + b'0') as char);
            } else {
                n3s.push((d as u8 - 10 + b'A') as char);
            }
            n3 /= 36;
        }
        format!("{}{}", n2s, n3s.iter().rev().collect::<String>())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concat_hex36() {
        assert_eq!(Solution::concat_hex36(13), "A91P1".to_string());
        assert_eq!(Solution::concat_hex36(36), "5101000".to_string());
    }
}
