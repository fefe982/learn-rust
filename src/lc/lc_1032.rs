// https://leetcode.com/problems/stream-of-characters/
// 1032. Stream of Characters

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct TrieNode {
    leaf: bool,
    children: HashMap<u8, Rc<RefCell<TrieNode>>>,
}
impl Default for TrieNode {
    fn default() -> Self {
        TrieNode {
            leaf: false,
            children: HashMap::new(),
        }
    }
}
impl TrieNode {
    fn step(&self, c: char) -> Option<Rc<RefCell<TrieNode>>> {
        self.children.get(&(c as u8)).cloned()
    }
}

struct StreamChecker {
    root: Rc<RefCell<TrieNode>>,
    last: Vec<Rc<RefCell<TrieNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let sc = StreamChecker {
            root: Rc::new(RefCell::new(TrieNode::default())),
            last: vec![],
        };
        for word in words {
            sc.insert(word);
        }
        sc
    }

    fn query(&mut self, letter: char) -> bool {
        let mut result = false;
        let mut new_last = vec![];
        self.last.push(self.root.clone());
        for q in &self.last {
            if let Some(n) = q.borrow().step(letter) {
                if n.borrow().leaf {
                    result = true;
                }
                new_last.push(n);
            }
        }
        self.last = new_last;
        result
    }

    fn insert(&self, word: String) {
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
}
/*
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stream_checker() {
        let mut sc = StreamChecker::new(vec![
            String::from("cd"),
            String::from("f"),
            String::from("kl"),
        ]);
        let query_char = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
        let query_res = [
            false, false, false, true, false, true, false, false, false, false, false, true,
        ];
        for (&q, &r) in query_char.iter().zip(&query_res) {
            assert_eq!(sc.query(q), r);
        }
    }
}
