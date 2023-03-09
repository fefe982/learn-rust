// https://leetcode.com/problems/regular-expression-matching/
// 10. Regular Expression Matching

pub struct Solution;
use std::collections::HashSet;
impl Solution {
    fn is_match_helper(
        s: &[u8],
        p: &[u8],
        idxs: usize,
        idxp: usize,
        cache: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if idxs == s.len() && idxp == p.len() {
            return true;
        }
        if idxs > s.len() || idxp >= p.len() {
            return false;
        }
        if cache.contains(&(idxs, idxp)) {
            return false;
        }
        let matched = if idxp + 1 < p.len() && p[idxp + 1] == b'*' {
            (idxs < s.len()
                && (p[idxp] == b'.' || p[idxp] == s[idxs])
                && Self::is_match_helper(s, p, idxs + 1, idxp, cache))
                || Self::is_match_helper(s, p, idxs, idxp + 2, cache)
        } else {
            idxs < s.len()
                && (s[idxs] == p[idxp] || p[idxp] == b'.')
                && Self::is_match_helper(s, p, idxs + 1, idxp + 1, cache)
        };
        if !matched {
            cache.insert((idxs, idxp));
        }
        matched
    }
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut cache: HashSet<(usize, usize)> = HashSet::new();
        Self::is_match_helper(s, p, 0usize, 0usize, &mut cache)
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
            Solution::is_match(String::from("aa"), String::from("a*")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("ab"), String::from(".*")),
            true
        );
    }
}
