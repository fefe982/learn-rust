// https://leetcode.com/problems/longest-repeating-character-replacement/
// 424. Longest Repeating Character Replacement
pub struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut i = 0;
        let mut c = [0; 26];
        let s = s.as_bytes();
        let mut max = 0;
        let mut res = 0;
        for j in 0..s.len() {
            let idx = (s[j] - b'A') as usize;
            c[idx] += 1;
            max = max.max(c[idx]);
            if (j - i + 1) as i32 - max > k {
                c[(s[i] - b'A') as usize] -= 1;
                i += 1;
            }
            res = res.max(j - i + 1);
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_character_replacement() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
