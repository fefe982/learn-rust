// https://leetcode.com/problems/find-and-replace-pattern/
// 890. Find and Replace Pattern
pub struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn cnt(s: &String) -> Vec<i32> {
            let mut t = [0; 26];
            let n = s.len();
            let mut v = Vec::with_capacity(n);
            let mut i = 0;
            for c in s.chars() {
                let ic = (c as u8 - b'a') as usize;
                t[ic] += 1;
                if t[ic] == 1 {
                    t[ic] += i * n as i32;
                    i += 1;
                }
                v.push(t[ic]);
            }
            v
        }
        let t = cnt(&pattern);
        let mut r = vec![];
        for s in words {
            if cnt(&s) == t {
                r.push(s);
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_and_replace_pattern() {
        assert_sort_eq!(
            Solution::find_and_replace_pattern(vec_str!["abc", "deq", "mee", "aqq", "dkd", "ccc"], "abb".to_string()),
            ["mee", "aqq"]
        );
        assert_sort_eq!(
            Solution::find_and_replace_pattern(vec_str!["a", "b", "c"], "a".to_string()),
            ["a", "b", "c"]
        );
    }
}
