// https://leetcode.com/problems/count-prefix-and-suffix-pairs-ii/
// 3045. Count Prefix and Suffix Pairs II
pub struct Solution;
struct Trie {
    cnt: i32,
    children: std::collections::HashMap<i32, Trie>,
}
impl Trie {
    fn new() -> Self {
        Self {
            cnt: 0,
            children: std::collections::HashMap::new(),
        }
    }
    fn insert_u8(&mut self, s: &[u8], i: usize) -> i64 {
        if i == s.len() {
            self.cnt += 1;
            self.cnt as i64 - 1
        } else {
            let idx = (s[i] - 'a' as u8) as i32 * 26 + (s[s.len() - i - 1] - 'a' as u8) as i32;
            self.cnt as i64
                + self
                    .children
                    .entry(idx)
                    .or_insert_with(|| Trie::new())
                    .insert_u8(s, i + 1)
        }
    }
}
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut trie = Trie::new();
        let mut ans = 0;
        for w in words.iter() {
            ans += trie.insert_u8(w.as_bytes(), 0);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_prefix_suffix_pairs() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec_str!["a", "aba", "ababa", "aa"]),
            4
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec_str!["pa", "papa", "ma", "mama"]),
            2
        );
        assert_eq!(Solution::count_prefix_suffix_pairs(vec_str!["abab", "ab"]), 0);
    }
}
