// https://leetcode.com/problems/partition-string/
// 3597. Partition String
pub struct Solution;
#[derive(Default)]
struct Trie {
    children: std::collections::HashMap<char, Trie>,
}
impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        let mut trie = Trie::default();
        let mut ans = vec![];
        let mut node = &mut trie;
        let mut ss = "".to_string();
        for c in s.chars() {
            ss.push(c);
            if node.children.contains_key(&c) {
                node = node.children.get_mut(&c).unwrap();
            } else {
                ans.push(ss);
                ss = "".to_string();
                node.children.insert(c, Trie::default());
                node = &mut trie;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_string() {
        assert_eq!(
            Solution::partition_string("abbccccd".to_string()),
            vec!["a", "b", "bc", "c", "cc", "d"]
        );
        assert_eq!(Solution::partition_string("aaaa".to_string()), vec!["a", "aa"]);
    }
}
