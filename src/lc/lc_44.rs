// https://leetcode.com/problems/wildcard-matching/
// 44. Wildcard Matching
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    fn is_match_str(s: &[u8], p: &[u8], cache: &mut HashSet<(usize, usize)>) -> bool {
        if cache.contains(&(s.len(), p.len())) {
            return false;
        }
        if p.len() == 0 {
            return s.len() == 0;
        }
        let res = match p[0] {
            b'?' => s.len() > 0 && Self::is_match_str(&s[1..], &p[1..], cache),
            b'*' => {
                (s.len() > 0 && Self::is_match_str(&s[1..], p, cache))
                    || Self::is_match_str(s, &p[1..], cache)
            }
            _ => s.len() > 0 && s[0] == p[0] && Self::is_match_str(&s[1..], &p[1..], cache),
        };
        cache.insert((s.len(), p.len()));
        res
    }
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_str(s.as_bytes(), p.as_bytes(), &mut HashSet::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_match() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("*")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("cb"), String::from("?a")),
            false
        );
    }
}
