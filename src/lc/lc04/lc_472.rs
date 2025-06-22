// https://leetcode.com/problems/concatenated-words/
// 472. Concatenated Words
pub struct Solution;
pub struct Trie {
    leaf: bool,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children.entry(s[0]).or_default().insert(&s[1..]);
        }
    }
    pub fn search(&self, s: &[u8]) -> i32 {
        self.search_helper(self, s, &mut std::collections::HashSet::new())
    }
    fn search_helper(&self, root: &Trie, s: &[u8], failed_pos: &mut std::collections::HashSet<usize>) -> i32 {
        if s.is_empty() {
            if self.leaf {
                1
            } else {
                -1
            }
        } else {
            if self.leaf {
                if !failed_pos.contains(&s.len()) {
                    let c = root.search_helper(root, s, failed_pos);
                    if c >= 0 {
                        return c + 1;
                    } else {
                        failed_pos.insert(s.len());
                    }
                }
            }
            if let Some(child) = self.children.get(&s[0]) {
                child.search_helper(root, &s[1..], failed_pos)
            } else {
                -1
            }
        }
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
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut t = Trie::default();
        for w in &words {
            t.insert(w.as_bytes());
        }
        let mut res = vec![];
        for w in words {
            if t.search(w.as_bytes()) > 1 {
                res.push(w);
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
    fn find_all_concatenated_words_in_a_dict() {
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec_str![
                "cat",
                "cats",
                "catsdogcats",
                "dog",
                "dogcatsdog",
                "hippopotamuses",
                "rat",
                "ratcatdogcat"
            ]),
            vec_str!["catsdogcats", "dogcatsdog", "ratcatdogcat"]
        );
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec_str!["cat", "dog", "catdog"]),
            vec_str!["catdog"]
        );
    }
}
