// https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// 467. Unique Substrings in Wraparound String
pub struct Solution;
impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let mut cnt = [0; 26];
        let mut last = -1;
        let mut len = 0;
        for &c in s.as_bytes().iter().chain([b'~'].iter()) {
            let c = (c - b'a') as i32;
            if last >= 0 && (last + 1) % 26 == c {
                len += 1;
            } else {
                if last >= 0 {
                    for i in (1..=len).rev().take(26) {
                        let s = ((last + 26 - (i % 26) + 1) % 26) as usize;
                        cnt[s] = cnt[s].max(i);
                    }
                }
                len = 1;
            }
            last = c;
        }
        cnt.iter().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_substring_in_wrapround_string() {
        assert_eq!(Solution::find_substring_in_wrapround_string("a".to_string()), 1);
        assert_eq!(Solution::find_substring_in_wrapround_string("cac".to_string()), 2);
        assert_eq!(Solution::find_substring_in_wrapround_string("zab".to_string()), 6);
    }
}
