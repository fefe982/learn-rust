// https://leetcode.com/problems/implement-trie-prefix-tree/
// 208. Implement Trie (Prefix Tree)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct TrieNode {
    leaf: bool,
    children: HashMap<u8, Rc<RefCell<TrieNode>>>,
}
pub struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode {
                leaf: false,
                children: HashMap::new(),
            })),
        }
    }

    pub fn insert(&self, word: String) {
        let mut node = self.root.clone();
        for c in word.as_bytes().iter() {
            let c_node = node.clone();
            let mut mut_node = c_node.borrow_mut();
            if let None = mut_node.children.get(c) {
                mut_node.children.insert(
                    *c,
                    Rc::new(RefCell::new(TrieNode {
                        leaf: false,
                        children: HashMap::new(),
                    })),
                );
            };
            node = mut_node.children.get(c).unwrap().clone();
        }
        node.borrow_mut().leaf = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = self.root.clone();
        for c in word.as_bytes().iter() {
            if let Some(n_node) = node.clone().borrow().children.get(c) {
                node = n_node.clone();
            } else {
                return false;
            }
        }
        node.clone().borrow().leaf
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = self.root.clone();
        for c in prefix.as_bytes().iter() {
            if let Some(n_node) = node.clone().borrow().children.get(c) {
                node = n_node.clone();
            } else {
                return false;
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trie() {
        let trie = Trie::new();
        trie.insert(String::from("apple"));
        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), false);
        assert_eq!(trie.starts_with(String::from("app")), true);
        trie.insert(String::from("app"));
        assert_eq!(trie.search(String::from("app")), true);
    }
}
