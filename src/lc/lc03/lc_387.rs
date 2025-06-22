// https://leetcode.com/problems/first-unique-character-in-a-string/
// 387. First Unique Character in a String
pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if cnt[(c as u8 - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_uniq_char() {
        assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
        assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
        assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
    }
}
