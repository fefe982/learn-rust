// https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/
// 2309. Greatest English Letter in Upper and Lower Case
pub struct Solution;
impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut lower = [false; 26];
        let mut upper = [false; 26];
        for c in s.chars() {
            if c.is_ascii_lowercase() {
                lower[(c as u8 - b'a') as usize] = true;
            } else if c.is_ascii_uppercase() {
                upper[(c as u8 - b'A') as usize] = true;
            }
        }
        for i in (0..26).rev() {
            if lower[i] && upper[i] {
                return ((b'A' + i as u8) as char).to_string();
            }
        }
        String::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greatest_letter() {
        assert_eq!(Solution::greatest_letter("lEeTcOdE".to_string()), "E");
        assert_eq!(Solution::greatest_letter("arRAzFif".to_string()), "R");
        assert_eq!(Solution::greatest_letter("AbCdEfGhIjK".to_string()), "");
    }
}
