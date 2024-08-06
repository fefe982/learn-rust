// https://leetcode.com/problems/sum-of-prefix-scores-of-strings/
// 2416. Sum of Prefix Scores of Strings
pub struct Solution;
#[derive(Default)]
struct Trie {
    cnt: i32,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    fn new() -> Self {
        Self {
            cnt: 0,
            children: std::collections::HashMap::new(),
        }
    }
    fn insert(&mut self, s: &str) {
        self.insert_u8(s.as_bytes())
    }
    fn insert_u8(&mut self, s: &[u8]) {
        self.cnt += 1;
        if s.len() > 0 {
            self.children.entry(s[0]).or_default().insert_u8(&s[1..]);
        }
    }
    fn search(&self, s: &[u8]) -> i32 {
        let mut res = self.cnt;
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                res += child.search(&s[1..])
            }
        }
        res
    }
}
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }
        words
            .into_iter()
            .map(|word| trie.search(word.as_bytes()) - trie.cnt)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_sum_prefix_scores() {
        assert_eq!(
            Solution::sum_prefix_scores(vec_str!["abc", "ab", "bc", "b"]),
            [5, 4, 3, 2]
        );
        assert_eq!(Solution::sum_prefix_scores(vec_str!["abcd"]), [4]);
    }
}
