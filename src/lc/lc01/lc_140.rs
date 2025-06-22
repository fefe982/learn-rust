// https://leetcode.com/problems/word-break-ii/
// 140. Word Break II
pub struct Solution;
use std::{collections::HashMap, str::from_utf8};
pub struct Trie {
    leaf: bool,
    children: HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: &str) {
        self.insert_u8(s.as_bytes())
    }
    pub fn insert_u8(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children.entry(s[0]).or_default().insert_u8(&s[1..]);
        }
    }
    pub fn common_prefix_search(&self, s: &[u8]) -> Vec<usize> {
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
            children: HashMap::new(),
        }
    }
}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut word_trie = Trie::default();
        for word in word_dict {
            word_trie.insert(&word);
        }
        let mut subs: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut pos = vec![0usize];
        let s_u8 = s.as_bytes();
        subs.entry(0).or_default();
        while let Some(p) = pos.pop() {
            if p == s_u8.len() {
                continue;
            }
            for n in word_trie.common_prefix_search(&s_u8[p..]) {
                let v = subs.entry(n + p).or_default();
                if v.len() == 0 {
                    pos.push(n + p);
                }
                v.push(p);
            }
        }
        if let None = subs.get(&s_u8.len()) {
            return Vec::<String>::new();
        }
        let mut res = Vec::new();
        let mut partition = Vec::new();
        partition.push((s_u8.len(), 0usize));
        while let Some(&(pos, index)) = partition.last() {
            let next_vec = subs.get(&pos).unwrap();
            if index >= next_vec.len() {
                partition.pop();
                if partition.len() == 0 {
                    break;
                }
                if let Some((_, index)) = partition.last_mut() {
                    *index += 1;
                }
                continue;
            }
            partition.push((next_vec[index], 0));
            if next_vec[index] == 0 {
                let mut word_str_u8 = Vec::new();
                for i in (1..partition.len()).rev() {
                    word_str_u8.extend_from_slice(&s_u8[partition[i].0..partition[i - 1].0]);
                    if i != 1 {
                        word_str_u8.push(' ' as u8);
                    }
                }
                res.push(from_utf8(&word_str_u8).unwrap().to_owned())
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn word_break() {
        assert_eq!(
            HashSet::<_>::from_iter(Solution::word_break(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "and".to_string(),
                    "sand".to_string(),
                    "dog".to_string()
                ]
            )),
            HashSet::<_>::from_iter(vec!["cats and dog".to_string(), "cat sand dog".to_string()])
        );
        assert_eq!(
            HashSet::<_>::from_iter(Solution::word_break(
                "pineapplepenapple".to_string(),
                vec![
                    "apple".to_string(),
                    "pen".to_string(),
                    "applepen".to_string(),
                    "pine".to_string(),
                    "pineapple".to_string()
                ]
            )),
            HashSet::<_>::from_iter(vec![
                "pine apple pen apple".to_string(),
                "pineapple pen apple".to_string(),
                "pine applepen apple".to_string()
            ])
        );
        assert_eq!(
            HashSet::<_>::from_iter(Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            )),
            HashSet::<String>::new()
        );
    }
}
