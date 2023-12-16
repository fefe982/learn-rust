// https://leetcode.com/problems/valid-anagram/
// 242. Valid Anagram
pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut v = vec![0; 26];
        for &c in s.as_bytes() {
            v[(c - b'a') as usize] += 1;
        }
        for &c in t.as_bytes() {
            let idx = (c - b'a') as usize;
            v[idx] -= 1;
            if v[idx] < 0 {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_anagram() {
        assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
