// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/
// 3043. Find the Length of the Longest Common Prefix
pub struct Solution;
struct Trie {
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: &[u8]) {
        if s.len() > 0 {
            self.children.entry(s[0]).or_default().insert(&s[1..]);
        }
    }
    pub fn longest_prefix(&self, s: &[u8]) -> i32 {
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                return child.longest_prefix(&s[1..]) + 1;
            }
        }
        0
    }
}
impl Default for Trie {
    fn default() -> Self {
        Trie {
            children: std::collections::HashMap::new(),
        }
    }
}
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::default();
        for i in arr1.into_iter() {
            trie.insert(i.to_string().as_bytes());
        }
        let mut max = 0;
        for i in arr2.into_iter() {
            max = max.max(trie.longest_prefix(i.to_string().as_bytes()));
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]), 3);
        assert_eq!(Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]), 0);
    }
}
