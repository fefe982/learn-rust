// https://leetcode.com/problems/longest-string-chain/
// 1048. Longest String Chain
pub struct Solution;
impl Solution {
    pub fn pred(s: &[u8], l: &[u8]) -> bool {
        let mut j = 0usize;
        for i in 0..s.len() {
            if s[i] == l[j] {
                j += 1;
            } else if i == j && s[i] == l[j + 1] {
                j += 2;
            } else {
                return false;
            }
        }
        true
    }
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|w| w.len());
        let mut max_v = vec![1; words.len()];
        let mut max = 1;
        for idx in 1..words.len() {
            let w = words[idx].as_bytes();
            let wl = w.len();
            for prev in (0..idx).rev() {
                let pw = words[prev].as_bytes();
                let pwl = pw.len();
                if pwl == wl {
                    continue;
                }
                if pwl + 1 < wl {
                    break;
                }
                if Self::pred(pw, w) {
                    max_v[idx] = std::cmp::max(max_v[idx], max_v[prev] + 1);
                    max = std::cmp::max(max_v[idx], max);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn longest_str_chain() {
        assert_eq!(
            Solution::longest_str_chain(vec_str!["a", "b", "ba", "bca", "bda", "bdca"]),
            4
        );
        assert_eq!(
            Solution::longest_str_chain(vec_str!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]),
            5
        );
        assert_eq!(Solution::longest_str_chain(vec_str!["abcd", "dbqca"]), 1);
    }
}
