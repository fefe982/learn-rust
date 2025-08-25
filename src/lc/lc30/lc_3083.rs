// https://leetcode.com/problems/existence-of-a-substring-in-a-string-and-its-reverse/
// 3083. Existence of a Substring in a String and Its Reverse
pub struct Solution;
impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut cnt = [[0; 26]; 26];
        let s = s.as_bytes();
        for i in 1..s.len() {
            let c = (s[i] - b'a') as usize;
            let lc = (s[i - 1] - b'a') as usize;
            cnt[lc][c] = 1;
            if cnt[c][lc] == 1 {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_substring_present() {
        assert_eq!(Solution::is_substring_present("leetcode".to_string()), true);
        assert_eq!(Solution::is_substring_present("abcba".to_string()), true);
        assert_eq!(Solution::is_substring_present("abcd".to_string()), false);
    }
}
