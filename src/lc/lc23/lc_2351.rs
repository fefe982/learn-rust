// https://leetcode.com/problems/first-letter-to-appear-twice/
// 2351. First Letter to Appear Twice
pub struct Solution;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen = [false; 26];
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            if seen[i] {
                return c;
            }
            seen[i] = true;
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_repeated_character() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}
