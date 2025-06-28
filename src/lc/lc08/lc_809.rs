// https://leetcode.com/problems/expressive-words/
// 809. Expressive Words
pub struct Solution;
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        'w: for w in words {
            let w = w.as_bytes();
            if w.len() > s.len() {
                continue;
            }
            let mut last = b' ';
            let mut i = 0;
            let mut j = 0;
            let mut cw = 0;
            while i < s.len() {
                if j < w.len() && s[i] == w[j] {
                    if s[i] == last {
                        cw += 1;
                    } else {
                        cw = 1;
                    }
                    i += 1;
                    j += 1;
                } else {
                    if s[i] != last {
                        continue 'w;
                    }
                    while i < s.len() && s[i] == last {
                        i += 1;
                        cw += 1;
                    }
                    if cw < 3 {
                        continue 'w;
                    }
                    cw = 0;
                }
                last = s[i - 1];
            }
            if i == s.len() && j == w.len() {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn expressive_words() {
        assert_eq!(Solution::expressive_words("abcd".to_string(), vec_str!["abc"]), 0);
        assert_eq!(
            Solution::expressive_words("heeellooo".to_string(), vec_str!["hello", "hi", "helo"]),
            1
        );
        assert_eq!(
            Solution::expressive_words("zzzzzyyyyy".to_string(), vec_str!["zzyy", "zy", "zyy"]),
            3
        );
    }
}
