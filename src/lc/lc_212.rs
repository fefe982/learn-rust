// https://leetcode.com/problems/word-search-ii/
// 212. Word Search II
pub struct Solution;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct TrieNode {
    leaf: bool,
    children: HashMap<u8, Rc<RefCell<TrieNode>>>,
}
impl TrieNode {
    fn insert(&mut self, s: &str) {
        self.insert_u8(s.as_bytes())
    }
    fn insert_u8(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children
                .entry(s[0])
                .or_default()
                .clone()
                .borrow_mut()
                .insert_u8(&s[1..]);
        }
    }
    fn adv(&mut self, c: u8) -> Option<Rc<RefCell<TrieNode>>> {
        let mut fnd = self.children.get(&c);
        if let Some(t) = fnd.cloned() {
            let t = t.as_ref().borrow();
            if t.children.is_empty() && !t.leaf {
                fnd = None;
                self.children.remove(&c);
            }
        }
        fnd.cloned()
    }
}
impl Default for TrieNode {
    fn default() -> Self {
        TrieNode {
            leaf: false,
            children: HashMap::new(),
        }
    }
}
impl Solution {
    fn search(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        words: Rc<RefCell<TrieNode>>,
        word: &mut Vec<u8>,
        res: &mut Vec<String>,
    ) {
        if i >= board.len() || j >= board[0].len() || board[i][j] == '.' {
            return;
        }
        let c = board[i][j];
        if let Some(next) = words.as_ref().borrow_mut().adv(c as u8) {
            board[i][j] = '.';
            word.push(c as u8);
            {
                let mut n = next.borrow_mut();
                if n.leaf {
                    res.push(String::from_utf8(word.to_vec()).unwrap());
                    n.leaf = false;
                }
            }
            Self::search(board, i + 1, j, next.clone(), word, res);
            Self::search(board, i, j + 1, next.clone(), word, res);
            Self::search(board, i.wrapping_sub(1), j, next.clone(), word, res);
            Self::search(board, i, j.wrapping_sub(1), next, word, res);
            word.pop();
            board[i][j] = c;
        }
    }
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let tree = Rc::new(RefCell::new(TrieNode::default()));
        for w in words {
            tree.as_ref().borrow_mut().insert(&w);
        }
        let mut res = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::search(&mut board, i, j, tree.clone(), &mut Vec::new(), &mut res);
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
    fn find_words() {
        assert_eq!(
            Solution::find_words(
                vec_vec_chr![
                    ["o", "a", "b", "n"],
                    ["o", "t", "a", "e"],
                    ["a", "h", "k", "r"],
                    ["a", "f", "l", "v"]
                ],
                vec_str!["oa", "oaa"]
            ),
            vec_str!["oa", "oaa"]
        );
        assert_eq!(
            Solution::find_words(
                vec_vec_chr![
                    ["o", "a", "a", "n"],
                    ["e", "t", "a", "e"],
                    ["i", "h", "k", "r"],
                    ["i", "f", "l", "v"]
                ],
                vec_str!["oath", "pea", "eat", "rain"]
            ),
            vec_str!["oath", "eat"]
        );
        assert_eq!(
            Solution::find_words(vec_vec_chr![["a", "b"], ["c", "d"]], vec_str!["abcb"]),
            Vec::<String>::new()
        );
    }
}
