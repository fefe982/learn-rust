// https://leetcode.com/problems/palindrome-pairs/description/
// 336. Palindrome Pairs
pub struct Solution;
pub struct Trie {
    val: usize,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: &str, val: usize) {
        self.insert_u8(s.as_bytes().iter().cloned(), val)
    }
    pub fn insert_rev(&mut self, s: &str, val: usize) {
        self.insert_u8(s.as_bytes().iter().rev().cloned(), val)
    }
    pub fn insert_u8(&mut self, mut s: impl Iterator<Item = u8>, val: usize) {
        if let Some(c) = s.next() {
            self.children.entry(c).or_default().insert_u8(s, val);
        } else {
            self.val = val;
        }
    }
    pub fn common_prefix_search(&self, mut s: impl Iterator<Item = u8>) -> Vec<usize> {
        let mut res = Vec::new();
        if let Some(c) = s.next() {
            if let Some(child) = self.children.get(&c) {
                res = child.common_prefix_search(s);
            }
        }
        if self.val != usize::MAX {
            res.push(self.val);
        }
        res
    }
}
impl Default for Trie {
    fn default() -> Self {
        Trie {
            val: usize::MAX,
            children: std::collections::HashMap::new(),
        }
    }
}
impl Solution {
    fn check_palindrome(s: &[u8]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut t = Trie::default();
        let mut trev = Trie::default();
        for (i, word) in words.iter().enumerate() {
            t.insert(word, i);
            trev.insert_rev(word, i);
        }
        let mut res = vec![];
        for (i, word) in words.iter().enumerate() {
            let v = t.common_prefix_search(word.as_bytes().iter().cloned().rev());
            for il in v {
                if words[i].as_bytes().len() == words[il].as_bytes().len() && il <= i {
                    continue;
                }
                if Self::check_palindrome(
                    &word.as_bytes()[0..(words[i].as_bytes().len() - words[il].as_bytes().len())],
                ) {
                    res.push(vec![il as i32, i as i32]);
                }
            }
            let v = trev.common_prefix_search(word.as_bytes().iter().cloned());
            for il in v {
                if words[i].as_bytes().len() == words[il].as_bytes().len() && il <= i {
                    continue;
                }
                if Self::check_palindrome(&word.as_bytes()[words[il].as_bytes().len()..]) {
                    res.push(vec![i as i32, il as i32]);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn palindrome_pairs() {
        assert_eq!(
            Solution::palindrome_pairs(vec_str!["abcd", "dcba", "lls", "s", "sssll"]),
            vec_vec![[1, 0], [0, 1], [3, 2], [2, 4]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec_str!["bat", "tab", "cat"]),
            vec_vec![[1, 0], [0, 1]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec_str!["a", ""]),
            vec_vec![[1, 0], [0, 1]]
        );
    }
}
