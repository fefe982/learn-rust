// https://leetcode.com/problems/word-break/
// 139. Word Break
pub struct Solution;
struct Trie {
    leaf: bool,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    fn insert(&mut self, s: &str) {
        self.insert_u8(s.as_bytes())
    }
    fn insert_u8(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children.entry(s[0]).or_default().insert_u8(&s[1..]);
        }
    }
    fn common_prefix_search(&self, s: &[u8]) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                res = child.common_prefix_search(&s[1..])
            }
        }
        for r in res.iter_mut() {
            *r += 1;
        }
        if self.leaf {
            res.push(0usize);
        }
        res
    }
}
impl Default for Trie {
    fn default() -> Self {
        Trie {
            leaf: false,
            children: std::collections::HashMap::new(),
        }
    }
}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut word_trie = Trie::default();
        for word in word_dict {
            word_trie.insert(&word);
        }
        let mut subs = vec![false; s.as_bytes().len()];
        let mut pos = vec![0usize];
        let s_u8 = s.as_bytes();
        while let Some(p) = pos.pop() {
            for n in word_trie.common_prefix_search(&s_u8[p..]) {
                if n + p == s_u8.len() {
                    return true;
                }
                if !subs[n + p] {
                    subs[n + p] = true;
                    pos.push(n + p);
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn word_break() {
        assert_eq!(
            Solution::word_break("leetcode".to_string(), vec_str!["leet", "code"]),
            true
        );
        assert_eq!(
            Solution::word_break(
                "pineapplepenapple".to_string(),
                vec_str!["apple", "pen", "applepen", "pine", "pineapple"]
            ),
            true
        );
        assert_eq!(
            Solution::word_break("catsandog".to_string(), vec_str!["cats", "dog", "sand", "and", "cat"]),
            false
        );
    }
}
