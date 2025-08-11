// https://leetcode.com/problems/length-of-the-longest-valid-substring/
// 2781. Length of the Longest Valid Substring
pub struct Solution;
struct Trie {
    leaf: bool,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: String) {
        self.insert_u8(s.as_bytes())
    }
    pub fn insert_u8(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children.entry(s[0]).or_default().insert_u8(&s[1..]);
        }
    }
    pub fn max_len_not_in(&self, s: &[u8]) -> i32 {
        if self.leaf {
            return -1;
        }
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                return child.max_len_not_in(&s[1..]) + 1;
            }
        }
        s.len() as i32
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
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let mut trie = Trie::default();
        for forbid in forbidden {
            trie.insert(forbid);
        }
        let mut len = vec![0; word.len()];
        let word = word.as_bytes();
        for i in 0..word.len() {
            len[i] = trie.max_len_not_in(&word[i..]) as usize;
        }
        let mut ans = 0;
        for i in 0..len.len() {
            let mut r = i + len[i];
            let mut j = i + 1;
            while j < r {
                r = r.min(j + len[j]);
                j += 1;
            }
            ans = ans.max(r - i);
            if word.len() - i < ans + 1 {
                break;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_longest_valid_substring() {
        assert_eq!(
            Solution::longest_valid_substring("cbaaaabc".to_string(), vec_str!["aaa", "cb"]),
            4
        );
        assert_eq!(
            Solution::longest_valid_substring("leetcode".to_string(), vec_str!["de", "le", "e"]),
            4
        );
    }
}
