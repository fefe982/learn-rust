// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// 3. Longest Substring Without Repeating Characters
pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut max_len = 0;
        let mut i = 0;
        let mut cnt = [0; 128];
        for j in 0..s.len() {
            let c = s[j] as usize;
            cnt[c] += 1;
            while cnt[c] > 1 {
                cnt[s[i] as usize] -= 1;
                i += 1;
            }
            max_len = max_len.max(j - i + 1);
        }
        max_len as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
