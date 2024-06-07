// https://leetcode.com/problems/replace-words/
// 648. Replace Words
pub struct Solution;
impl Solution {
    fn insert(tree: &mut Vec<(Vec<usize>, bool)>, word: &str) {
        let mut node = 0;
        for c in word.bytes() {
            let ci = (c - b'a') as usize;
            if tree[node].0[ci] == 0 {
                tree[node].0[ci] = tree.len();
                tree.push((vec![0; 26], false));
            }
            node = tree[node].0[ci];
        }
        tree[node].1 = true;
    }
    fn search(tree: &Vec<(Vec<usize>, bool)>, word: &str) -> String {
        let mut node = 0;
        for (i, c) in word.bytes().enumerate() {
            let ci = (c - b'a') as usize;
            if tree[node].0[ci] == 0 {
                return word.to_owned();
            }
            node = tree[node].0[ci];
            if tree[node].1 {
                return word[..i + 1].to_owned();
            }
        }
        return word.to_owned();
    }
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut tree = vec![(vec![0; 26], false)];
        for word in dictionary {
            Self::insert(&mut tree, &word);
        }
        sentence
            .split_ascii_whitespace()
            .map(|x| Self::search(&tree, x))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_replace_words() {
        assert_eq!(
            Solution::replace_words(
                vec_str!["cat", "bat", "rat"],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat"
        );
        assert_eq!(
            Solution::replace_words(vec_str!["a", "b", "c"], "aadsfasf absbs bbab cadsfafs".to_string()),
            "a a b c"
        );
    }
}
