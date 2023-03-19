// https://leetcode.com/problems/design-add-and-search-words-data-structure/
// 211. Design Add and Search Words Data Structure
use std::collections::HashMap;
struct Trie {
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
    pub fn search(&self, s: &[u8]) -> bool {
        if s.len() == 0 {
            self.leaf
        } else {
            let u8 = s[0];
            if u8 == b'.' {
                for v in self.children.values() {
                    if v.search(&s[1..]) {
                        return true;
                    }
                }
                false
            } else if let Some(v) = self.children.get(&u8) {
                v.search(&s[1..])
            } else {
                false
            }
        }
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
pub struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            trie: Trie::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    }

    pub fn search(&self, word: String) -> bool {
        self.trie.search(word.as_bytes())
    }
}

/*
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn word_dict() {
        let mut word_dict = WordDictionary::new();
        word_dict.add_word(String::from("bad"));
        word_dict.add_word(String::from("dad"));
        word_dict.add_word(String::from("mad"));
        assert_eq!(word_dict.search(String::from("pad")), false);
        assert_eq!(word_dict.search(String::from("bad")), true);
        assert_eq!(word_dict.search(String::from(".ad")), true);
        assert_eq!(word_dict.search(String::from("b..")), true);
    }
}
