// https://leetcode.com/problems/longest-common-suffix-queries/
// 3093. Longest Common Suffix Queries
pub struct Solution;
struct Trie {
    min_len: usize,
    idx: usize,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    fn new() -> Self {
        Self {
            min_len: usize::MAX,
            idx: usize::MAX,
            children: std::collections::HashMap::new(),
        }
    }
    fn insert_u8(&mut self, s: &[u8], si: usize, idx: usize) {
        if s.len() < self.min_len {
            self.min_len = s.len();
            self.idx = idx;
        }
        if si > 0 {
            self.children
                .entry(s[si - 1])
                .or_insert_with(|| Trie::new())
                .insert_u8(s, si - 1, idx);
        }
    }
    fn search(&self, s: &[u8], si: usize) -> usize {
        if si == 0 {
            return self.idx;
        }
        if let Some(child) = self.children.get(&s[si - 1]) {
            child.search(s, si - 1)
        } else {
            self.idx
        }
    }
}
impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut t = Trie::new();
        for (i, w) in words_container.into_iter().enumerate() {
            let wb = w.as_bytes();
            t.insert_u8(wb, wb.len(), i);
        }
        let mut res = Vec::with_capacity(words_query.len());
        for q in words_query {
            let qb = q.as_bytes();
            res.push(t.search(qb, qb.len()) as i32);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn string_indices() {
        assert_eq!(
            Solution::string_indices(vec_str!["abcd", "bcd", "xbcd"], vec_str!["cd", "bcd", "xyz"]),
            [1, 1, 1]
        );
        assert_eq!(
            Solution::string_indices(
                vec_str!["abcdefgh", "poiuygh", "ghghgh"],
                vec_str!["gh", "acbfgh", "acbfegh"]
            ),
            [2, 0, 2]
        );
    }
}
