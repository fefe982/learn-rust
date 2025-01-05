// https://leetcode.com/problems/shifting-letters-ii/
// 2381. Shifting Letters II
pub struct Solution;
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut shift = vec![0; s.len() + 1];
        for s in shifts {
            let o = if s[2] == 0 { -1 } else { 1 };
            shift[s[0] as usize] = (shift[s[0] as usize] + o) % 26;
            shift[s[1] as usize + 1] = (shift[s[1] as usize + 1] - o) % 26;
        }
        let mut o = 0;
        for i in 0..s.len() {
            o = (o + shift[i] + 26) % 26;
            s[i] = (s[i] - b'a' + o as u8) % 26 + b'a';
        }
        String::from_utf8(s).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_shifting_letters() {
        assert_eq!(
            Solution::shifting_letters("abc".to_string(), vec_vec![[0, 1, 0], [1, 2, 1], [0, 2, 1]]),
            "ace"
        );
        assert_eq!(
            Solution::shifting_letters("dztz".to_string(), vec_vec![[0, 0, 0], [1, 1, 1]]),
            "catz"
        );
    }
}
